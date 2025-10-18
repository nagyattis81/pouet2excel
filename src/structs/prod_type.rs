use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProdType {
    #[serde(rename = "demo")]
    _Demo,
    #[serde(rename = "64k")]
    _64k,
    #[serde(rename = "4k")]
    _4k,
    #[serde(rename = "invitation")]
    _Invitation,
    #[serde(rename = "musicdisk")]
    _Musicdisk,
    #[serde(rename = "cracktro")]
    _Cracktro,
    #[serde(rename = "intro")]
    _Intro,
    #[serde(rename = "dentro")]
    _Dentro,
    #[serde(rename = "game")]
    _Game,
    #[serde(rename = "16k")]
    _16k,
    #[serde(rename = "bbstro")]
    _Bbstro,
    #[serde(rename = "100k")]
    _100k,
    #[serde(rename = "diskmag")]
    _Diskmag,
    #[serde(rename = "40k")]
    _40k,
    #[serde(rename = "report")]
    _Report,
    #[serde(rename = "wild")]
    _Wild,
    #[serde(rename = "32k")]
    _32k,
}

const ALLOWED_TYPES: &[&str] = &[];

pub fn from_comma_separated_prod_types<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<ProdType>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    let Some(s) = s else { return Ok(None) };

    let parts = s
        .split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| {
            serde_json::from_str::<ProdType>(&format!("\"{}\"", p))
                .map_err(|_| DeError::unknown_variant(p, ALLOWED_TYPES))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Some(parts))
}
