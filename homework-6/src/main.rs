use homework_6::{config::hw6_config::HW6Parameters, questions::question_1::question_1};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

const CONFIG_PATH: &'static str = "homework-6\\config\\homework_6.toml";

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng: ThreadRng = rand::rng();
    let HW6Parameters {
        question_1_parameters_path,
        question_2_parameters_path,
        question_3_parameters_path,
        question_4_parameters_path,
        question_5_parameters_path,
        question_6_parameters_path,
    } = toml::from_str(&read_to_string(CONFIG_PATH.to_string())?)?;

    question_1(&mut rng)?;
    // question_2()?;
    // question_3()?;
    // question_4()?;
    // question_5()?;
    // question_6()?;
    Ok(())
}
