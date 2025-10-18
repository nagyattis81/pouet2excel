use serde::Deserialize;

use crate::structs::pouet_data_prod::PouetDataProd;

#[derive(Debug, Deserialize)]
pub struct PouetDataLatest {
    pub prods: Option<PouetDataProd>,
}
