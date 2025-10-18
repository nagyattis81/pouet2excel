use serde::Deserialize;

pub fn from_str_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                s.parse::<T>().map(Some).map_err(serde::de::Error::custom)
            }
        }
        None => Ok(None),
    }
}
