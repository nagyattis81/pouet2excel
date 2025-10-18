use serde::de::{Deserializer, Error as DeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProdType {
    #[serde(rename = "intro")]
    _Intro,

    #[serde(rename = "32b")]
    _32b,

    #[serde(rename = "64b")]
    _64b,

    #[serde(rename = "128b")]
    _128b,

    #[serde(rename = "256b")]
    _256b,

    #[serde(rename = "512b")]
    _512b,

    #[serde(rename = "1k")]
    _1k,

    #[serde(rename = "4k")]
    _4k,

    #[serde(rename = "8k")]
    _8k,

    #[serde(rename = "16k")]
    _16k,

    #[serde(rename = "32k")]
    _32k,

    #[serde(rename = "40k")]
    _40k,

    #[serde(rename = "64k")]
    _64k,

    #[serde(rename = "80k")]
    _80k,

    #[serde(rename = "96k")]
    _96k,

    #[serde(rename = "100k")]
    _100k,

    #[serde(rename = "128k")]
    _128k,

    #[serde(rename = "256k")]
    _256k,

    #[serde(rename = "demo")]
    _Demo,

    #[serde(rename = "demotool")]
    _Demotool,

    #[serde(rename = "demopack")]
    _Demopack,

    #[serde(rename = "artpack")]
    _Artpack,

    #[serde(rename = "fastdemo")]
    _Fastdemo,

    #[serde(rename = "procedural graphics")]
    _ProceduralGraphics,

    #[serde(rename = "invitation")]
    _Invitation,

    #[serde(rename = "liveact")]
    _Liveact,

    #[serde(rename = "votedisk")]
    _Votedisk,

    #[serde(rename = "musicdisk")]
    _Musicdisk,

    #[serde(rename = "cracktro")]
    _Cracktro,

    #[serde(rename = "dentro")]
    _Dentro,

    #[serde(rename = "game")]
    _Game,

    #[serde(rename = "bbstro")]
    _Bbstro,

    #[serde(rename = "diskmag")]
    _Diskmag,

    #[serde(rename = "report")]
    _Report,

    #[serde(rename = "wild")]
    _Wild,

    #[serde(rename = "slideshow")]
    _Slideshow,
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
