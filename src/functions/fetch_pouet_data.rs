use std::error::Error;

use crate::structs::pouet_data::PouetData;

pub async fn fetch_pouet_data() -> Result<PouetData, Box<dyn Error>> {
    let url = "https://data.pouet.net/json.php";
    let text = reqwest::get(url).await?.text().await?;
    let data: PouetData = serde_json::from_str(&text)?;
    println!("✅ Read {}", url);
    Ok(data)
}
