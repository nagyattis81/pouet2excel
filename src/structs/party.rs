use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    pub name: Option<String>,
    pub web: Option<String>,
}
