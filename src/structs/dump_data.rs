use serde::Deserialize;

use crate::structs::prod::Prod;

#[derive(Debug, Deserialize)]
pub struct DumpData {
    _dump_date: Option<String>,
    pub prods: Option<Vec<Prod>>,
}
