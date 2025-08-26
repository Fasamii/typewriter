use super::profile;
use serde::{Deserialize, ser::SerializeSeq};
use std::collections::HashMap;

pub fn serialize<S>(
    map: &HashMap<(char, char), profile::BigramStats>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serializer.serialize_seq(Some(map.len()))?;
    for (key, value) in map {
        seq.serialize_element(&(key, value))?;
    }
    seq.end()
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<HashMap<(char, char), profile::BigramStats>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec: Vec<((char, char), profile::BigramStats)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}
