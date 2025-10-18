use flate2::read::GzDecoder;
use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{copy, BufReader},
    path::Path,
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PouetData {
    latest: Option<PouetDataLatest>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PouetDataLatest {
    prods: Option<PouetDataProd>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PouetDataProd {
    filename: Option<String>,
    url: Option<String>,
    size_in_bytes: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    pub name: Option<String>,
    pub web: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(deserialize_with = "from_str_opt")]
    pub id: Option<u32>,
    pub name: Option<String>,
    pub acronym: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Placement {
    pub party: Option<Party>,
    #[serde(deserialize_with = "from_str_opt")]
    pub ranking: Option<u32>,
    #[serde(deserialize_with = "from_str_opt")]
    pub year: Option<u32>,
    pub compo_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadLink {
    #[serde(rename = "type")]
    pub download_link_type: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(deserialize_with = "from_str_opt")]
    pub id: Option<u32>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Credit {
    pub user: Option<User>,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProdType {
    #[serde(rename = "demo")]
    _Demo,
    #[serde(rename = "64k")]
    _64k,
    #[serde(rename = "4k")]
    _4k,
    #[serde(rename = "invitation")]
    _Invitation,
    #[serde(rename = "musicdisk")]
    _Musicdisk,
    #[serde(rename = "cracktro")]
    _Cracktro,
    #[serde(rename = "intro")]
    _Intro,
    #[serde(rename = "dentro")]
    _Dentro,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prod {
    #[serde(deserialize_with = "from_str_opt")]
    pub id: Option<u32>,
    pub name: Option<String>,

    #[serde(rename = "type", deserialize_with = "from_comma_separated_prod_types")]
    pub prod_type: Option<Vec<ProdType>>,

    pub download: Option<String>,
    pub screenshot: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(deserialize_with = "from_str_opt")]
    pub voteup: Option<u32>,
    #[serde(deserialize_with = "from_str_opt")]
    pub votepig: Option<u32>,
    #[serde(deserialize_with = "from_str_opt")]
    pub votedown: Option<u32>,
    #[serde(deserialize_with = "from_str_opt")]
    pub voteavg: Option<f32>,
    pub types: Option<Vec<String>>,
    pub groups: Option<Vec<Group>>,
    pub placings: Option<Vec<Placement>>,
    pub platforms: Option<HashMap<String, Platform>>,
    pub party: Option<Party>,
    #[serde(rename = "downloadLinks")]
    pub download_links: Option<Vec<DownloadLink>>,
    pub credits: Option<Vec<Credit>>,
}

#[derive(Debug, Deserialize)]
pub struct DumpData {
    pub dump_date: Option<String>,
    pub prods: Option<Vec<Prod>>,
}

// Az elvárt értékek listája a hibaüzenethez.
const ALLOWED_TYPES: &[&str] = &[
    "demo",
    "64k",
    "4k",
    "invitation",
    "game",
    "music",
    "wild",
    "intro",
];

fn from_comma_separated_prod_types<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<ProdType>>, D::Error>
where
    D: Deserializer<'de>, // <-- ITT IS FONTOS
{
    // Beolvassuk Option<String>-ként (lehet None is).
    let s = Option::<String>::deserialize(deserializer)?;
    let Some(s) = s else { return Ok(None) };

    // "demo,invitation" -> ["demo", "invitation"] -> Vec<ProdType>
    let parts = s
        .split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| {
            // Trükk: a serde-vel egyszerűen parse-oljuk az enumot,
            // mintha egy önálló string JSON lenne: "\"demo\"" -> ProdType::Demo
            serde_json::from_str::<ProdType>(&format!("\"{}\"", p))
                .map_err(|_| DeError::unknown_variant(p, ALLOWED_TYPES))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Some(parts))
}

fn from_str_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                s.parse::<T>().map(Some).map_err(serde::de::Error::custom)
            }
        }
        None => Ok(None),
    }
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

fn display_prods(json_filename: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(json_filename)?;
    let reader = BufReader::new(file);
    let dump: DumpData = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("❌ JSON read error: {}", err);
            return Ok(());
        }
    };
    let pretty = serde_json::to_string_pretty(&dump.prods)?;
    println!("{}", pretty);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json_filename = "test.json";

    if Path::new(json_filename).exists() {
        println!("✅ Found existing file: {}", json_filename);
        let _ = display_prods(json_filename.to_string());
        return Ok(());
    }

    let data = fetch_pouet_data().await?;

    let (download_url, filename) = match latest_dump(&data) {
        Some(t) => t,
        None => {
            eprintln!("❌ Not found! (url, filename) in JSON");
            return Ok(());
        }
    };

    download_to_file(&download_url, &filename).await?;
    println!("✅ Download {}", filename);

    let json_filename = format!("{}", filename.trim_end_matches(".gz"));
    decompress_gzip_file(&filename, &json_filename)?;
    println!("✅ Decompressed {}", json_filename);

    let _ = display_prods(json_filename);

    Ok(())
}
