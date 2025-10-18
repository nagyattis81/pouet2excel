use crate::structs::pouet_data::PouetData;

pub fn latest_dump(data: &PouetData) -> Option<(String, String)> {
    let prod = data.latest.as_ref()?.prods.as_ref()?;
    match (&prod.url, &prod.filename) {
        (Some(u), Some(f)) => Some((u.clone(), f.clone())),
        _ => None,
    }
}
