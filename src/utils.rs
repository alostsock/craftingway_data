use serde::Deserialize;
use std::fs::File;
use std::io::prelude::Write;

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
