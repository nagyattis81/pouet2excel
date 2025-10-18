use crate::functions::from_str_opt::from_str_opt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(deserialize_with = "from_str_opt")]
    pub id: Option<u32>,
    pub name: Option<String>,
    pub acronym: Option<String>,
}
