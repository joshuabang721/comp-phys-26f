use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{read_to_string, remove_file},
    io::Write,
    process::Command,
    time::Instant,
};

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

pub fn publish_matlab_script(script: &str, function: &str, pdfdoc: &str, paths: Vec<&str>) -> () {
    let matlab_start = Instant::now();
    println!("Matlab start!");

    let paths_path = "dataio\\paths.json";
    write_data_to_file(&paths, paths_path, FileDataType::Json);

    let args = [
        "-batch",
        &format!(
            "addpath(\\\"scripts\\\");addpath(\\\"output\\\");p = publish(\\\"{}\\\", format='pdf', outputDir='output', codeToEvaluate='{}(\\\"{}\\\")');movefile(p, fullfile(pwd, \\\"{}\\\"));",
            script, function, paths_path, pdfdoc
        ),
    ];
    let _ = Command::new("matlab").args(args).status();

    remove_file(paths_path).unwrap();

    let matlab_duration = matlab_start.elapsed();
    println!("Matlab finish! Time elapsed: {:?}", matlab_duration);
}

pub enum FileDataType {
    Json,
    Toml,
}
