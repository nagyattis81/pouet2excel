use std::collections::HashMap;

use crate::structs::credit::Credit;
use crate::structs::download_link::DownloadLink;
use crate::structs::group::Group;
use crate::structs::party::Party;
use crate::structs::placement::Placement;
use crate::structs::platform::Platform;
use crate::structs::prod_type::from_comma_separated_prod_types;
use crate::{functions::from_str_opt::from_str_opt, structs::prod_type::ProdType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Prod {
    #[serde(deserialize_with = "from_str_opt")]
    pub id: Option<u32>,

    pub name: Option<String>,

    #[serde(rename = "type", deserialize_with = "from_comma_separated_prod_types")]
    pub prod_type: Option<Vec<ProdType>>,

    pub download: Option<String>,

    #[serde(deserialize_with = "from_str_opt")]
    pub party_place: Option<u32>,

    #[serde(deserialize_with = "from_str_opt")]
    pub party_year: Option<u32>,

    pub screenshot: Option<String>,

    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,

    #[serde(rename = "addedDate")]
    pub added_date: Option<String>,

    #[serde(deserialize_with = "from_str_opt")]
    pub voteup: Option<u32>,

    #[serde(deserialize_with = "from_str_opt")]
    pub votepig: Option<u32>,

    #[serde(deserialize_with = "from_str_opt")]
    pub votedown: Option<u32>,

    #[serde(deserialize_with = "from_str_opt")]
    pub voteavg: Option<f32>,

    pub types: Option<Vec<String>>,

    pub groups: Option<Vec<Group>>,

    pub placings: Option<Vec<Placement>>,

    pub platforms: Option<HashMap<String, Platform>>,

    pub party: Option<Party>,

    #[serde(rename = "downloadLinks")]
    pub download_links: Option<Vec<DownloadLink>>,

    pub credits: Option<Vec<Credit>>,
}
