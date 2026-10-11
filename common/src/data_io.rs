use std::{
    error::Error,
    fs::{File, read_to_string},
    io::Write,
};

use serde::{Serialize, de::DeserializeOwned};

pub fn read_data_from_json<T: DeserializeOwned>(path: String) -> Result<T, Box<dyn Error>> {
    let text: String = read_to_string(path)?;
    let data: T = serde_json::from_str(text.as_str())?;
    Ok(data)
}

pub fn write_data_to_json<T: Serialize>(path: String, data: &T) -> Result<(), Box<dyn Error>> {
    let text: String = serde_json::to_string_pretty(data)?;
    let mut file: File = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
