use std::{error::Error, fs::File, io::copy, path::Path};

pub async fn download_to_file(url: &str, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let mut file = File::create(path)?;
    copy(&mut bytes.as_ref(), &mut file)?;
    Ok(())
}
