use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::Write;
use std::path::Path;

pub fn read_csv_data<RecordType>(path: impl AsRef<Path>) -> impl Iterator<Item = RecordType>
where
    RecordType: serde::de::DeserializeOwned,
{
    let reader = csv::Reader::from_path(path).unwrap();
    reader.into_deserialize::<RecordType>().map(|r| r.unwrap())
}

pub fn bool_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let b = String::deserialize(deserializer)?;
    match b.trim().to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom("invalid boolean string")),
    }
}

pub fn write_json_file<T>(data: &T, path: &str)
where
    T: serde::Serialize,
{
    let json = serde_json::to_string(&data).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(&json.into_bytes()).unwrap();
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
