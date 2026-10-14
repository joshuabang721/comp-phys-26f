use serde::{Serialize, de::DeserializeOwned};
use std::{fs::read_to_string, io::Write};

pub fn read_data_from_file<T>(path: String, file_type: FileDataType) -> T
where
    T: DeserializeOwned,
{
    match file_type {
        FileDataType::Json => serde_json::from_str(&read_to_string(path).unwrap()).unwrap(),
        FileDataType::Toml => toml::from_str(&read_to_string(path).unwrap()).unwrap(),
    }
}

pub fn write_data_to_file<T>(data: &T, path: String, file_type: FileDataType)
where
    T: Serialize,
{
    std::fs::File::create(path)
        .unwrap()
        .write_all(
            (match file_type {
                FileDataType::Json => serde_json::to_string_pretty(data).unwrap(),
                FileDataType::Toml => toml::to_string_pretty(data).unwrap(),
            })
            .as_bytes(),
        )
        .unwrap();
}

pub enum FileDataType {
    Json,
    Toml,
}
