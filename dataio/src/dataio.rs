use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fs::read_to_string, io::Write, process::Command};

pub fn read_data_from_file<T>(path: &str, file_type: FileDataType) -> T
where
    T: DeserializeOwned,
{
    match file_type {
        FileDataType::Json => serde_json::from_str(&read_to_string(path).unwrap()).unwrap(),
        FileDataType::Toml => toml::from_str(&read_to_string(path).unwrap()).unwrap(),
    }
}

pub fn write_data_to_file<T>(data: &T, path: &str, file_type: FileDataType)
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

pub fn publish_matlab_script(script: &str, function: &str, pdfdoc: &str, data_paths: &str) -> () {
    let args = [
        "-batch",
        &format!(
            r#"p = publish("{}",
                format='pdf',
                outputDir="output",
                codeToEvaluate="{}({})");
            movefile(p, fullfile(pwd, "output", '{}'));"#,
            script, function, data_paths, pdfdoc
        ),
    ];

    let _ = Command::new("matlab").args(args).status();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatlabDocParameters {
    pub script: String,
    pub function: String,
    pub pdfdoc: String,
}

pub enum FileDataType {
    Json,
    Toml,
}
