mod functions;
mod structs;

use std::{error::Error, path::Path};

use crate::functions::{
    decompress_gzip_file::decompress_gzip_file, download_to_file::download_to_file,
    export_prods::export_prods, fetch_pouet_data::fetch_pouet_data, latest_dump::latest_dump,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = fetch_pouet_data().await?;

    let (download_url, filename) = match latest_dump(&data) {
        Some(t) => t,
        None => {
            eprintln!("❌ Not found! (url, filename) in JSON");
            return Ok(());
        }
    };

    let json_filename = format!("{}", filename.trim_end_matches(".gz"));

    if Path::new(&json_filename).exists() {
        println!("ℹ️  Found existing file: {}", json_filename);
        let _ = export_prods(json_filename.to_string());
        return Ok(());
    }

    download_to_file(&download_url, &filename).await?;
    println!("✅ Download {}", filename);

    decompress_gzip_file(&filename, &json_filename)?;
    println!("✅ Decompressed {}", json_filename);

    let _ = export_prods(json_filename);

    Ok(())
}
