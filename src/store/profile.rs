use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path};

const STATS_FILE_NAME: &'static str = "stats";

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub min_wpm: usize,
    pub avg_wpm: usize,
    pub accuracy: usize,

    pub keys: HashMap<char, CharStats>,
    #[serde(with = "bigram_serializer")]
    pub bigrams: HashMap<(char, char), BigramStats>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CharStats {
    pub count: usize,
    pub correct: usize,
    pub avg_dwell: usize,
    pub min_dwell: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BigramStats {
    pub count: usize,
    pub correct: usize,
    pub avg_flight: usize,
    pub min_flight: usize,
}

mod bigram_serializer {

    use serde::{Deserialize, ser::SerializeSeq};
    use std::collections::HashMap;

    pub fn serialize<S>(
        map: &HashMap<(char, char), super::BigramStats>,
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
    ) -> Result<HashMap<(char, char), super::BigramStats>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<((char, char), super::BigramStats)> = Vec::deserialize(deserializer)?;
        Ok(vec.into_iter().collect())
    }
}

impl Profile {
    pub fn new() -> Self {
        Self {
            min_wpm: 0,
            avg_wpm: 0,
            accuracy: 0,
            keys: HashMap::new(),
            bigrams: HashMap::new(),
        }
    }

    pub fn save(&self, config_path: &path::PathBuf) -> Result<(), io::Error> {
        let json_string = serde_json::to_string_pretty(self)?;
        fs::write(config_path.join(STATS_FILE_NAME), json_string)?;
        Ok(())
    }

    pub fn load(config_path: &path::PathBuf) -> Result<Self, io::Error> {
        let json_string = fs::read_to_string(config_path.join(STATS_FILE_NAME))?;
        let profile: Profile = serde_json::from_str(&json_string)?;
        Ok(profile)
    }
}

impl CharStats {
    pub fn new() -> Self {
        Self {
            count: 0,
            correct: 0,
            avg_dwell: 0,
            min_dwell: usize::MAX,
        }
    }

    pub fn accuracy(self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.correct as f64 / self.count as f64) * 100.0
        }
    }
}

impl BigramStats {
    pub fn new() -> Self {
        BigramStats {
            count: 0,
            correct: 0,
            avg_flight: 0,
            min_flight: usize::MAX,
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.correct as f64 / self.count as f64) * 100.0
        }
    }
}
