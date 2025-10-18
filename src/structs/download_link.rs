use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadLink {
    #[serde(rename = "type")]
    pub download_link_type: Option<String>,
    pub link: Option<String>,
}
