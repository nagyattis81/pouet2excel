use crate::functions::from_str_opt::from_str_opt;
use crate::structs::party::Party;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Placement {
    pub party: Option<Party>,
    #[serde(deserialize_with = "from_str_opt")]
    pub ranking: Option<u32>,
    #[serde(deserialize_with = "from_str_opt")]
    pub year: Option<u32>,
    pub compo_name: Option<String>,
}
