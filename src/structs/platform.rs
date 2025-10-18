use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Platform {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub slug: Option<String>,
}
