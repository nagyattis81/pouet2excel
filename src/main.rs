use serde::Deserialize;
use std::error::Error;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://data.pouet.net/json.php";
    let text = reqwest::get(url).await?.text().await?;
    let data: PouetData = serde_json::from_str(&text)?;
    println!("{:#?}", data);
    Ok(())
}
