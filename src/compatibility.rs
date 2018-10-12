extern crate serde;

use serde::{Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

// Pending serde-rs/serde#723
pub fn deserialize_option_of_fromstr_or_string<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum FromStrOrString<T> {
        FromStr(T),
        String(String),
    }

    match FromStrOrString::deserialize(deserializer)? {
        FromStrOrString::FromStr(x) => Ok(Some(x)),
        FromStrOrString::String(s) => s
            .parse()
            .map(Option::Some)
            .map_err(serde::de::Error::custom),
    }
}
