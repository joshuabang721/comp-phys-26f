use crate::config::q1_config::Q1Parameters;
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_1(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q1Parameters {} = toml::from_str(&read_to_string(params_path)?)?;

    Ok(())
}
