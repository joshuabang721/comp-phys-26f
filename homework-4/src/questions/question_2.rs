use crate::config::q2_config::Q2Parameters;
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_2(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q2Parameters {
        program_parameters: pgm,
        simulation_parameters: sim,
        distribution_parameters: dst,
        figure_parameters: fig,
    } = toml::from_str(&read_to_string(params_path)?)?;
    Ok(())
}
