use crate::functions::from_str_opt::from_str_opt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(deserialize_with = "from_str_opt")]
    pub id: Option<u32>,
    pub nickname: Option<String>,
}
