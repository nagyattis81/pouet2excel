use flate2::read::GzDecoder;
use serde::Deserialize;
use std::{error::Error, fs::File, io::copy, path::Path};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PouetData {
    latest: Option<Latest>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Latest {
    prods: Option<Prod>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Prod {
    filename: Option<String>,
    url: Option<String>,
    size_in_bytes: Option<u32>,
}

async fn fetch_pouet_data() -> Result<PouetData, Box<dyn Error>> {
    let url = "https://data.pouet.net/json.php";
    let text = reqwest::get(url).await?.text().await?;
    let data: PouetData = serde_json::from_str(&text)?;
    println!("✅ Read {}", url);
    Ok(data)
}

fn latest_dump(data: &PouetData) -> Option<(String, String)> {
    let prod = data.latest.as_ref()?.prods.as_ref()?;
    match (&prod.url, &prod.filename) {
        (Some(u), Some(f)) => Some((u.clone(), f.clone())),
        _ => None,
    }
}

async fn download_to_file(url: &str, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let mut file = File::create(path)?;
    copy(&mut bytes.as_ref(), &mut file)?;
    Ok(())
}

fn decompress_gzip_file(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> std::io::Result<()> {
    let input = File::open(src)?;
    let mut decoder = GzDecoder::new(input);
    let mut output = File::create(dest)?;
    copy(&mut decoder, &mut output)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = fetch_pouet_data().await?;

    if let Some((download_url, filename)) = latest_dump(&data) {
        download_to_file(&download_url, &filename).await?;
        println!("✅ Download {}", filename);

        if filename.ends_with(".gz") {
            let out_name = filename.trim_end_matches(".gz");
            let json_filename = format!("{}", out_name);
            decompress_gzip_file(&filename, &json_filename)?;
            println!("✅ Decompressed {}", json_filename);
        }
    }

    Ok(())
}
