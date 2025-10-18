use serde::Deserialize;

use crate::structs::pouet_data_latest::PouetDataLatest;

#[derive(Debug, Deserialize)]
pub struct PouetData {
    pub latest: Option<PouetDataLatest>,
}
