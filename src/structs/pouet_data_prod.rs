use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PouetDataProd {
    pub filename: Option<String>,
    pub url: Option<String>,
    _size_in_bytes: Option<u32>,
}
