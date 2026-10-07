use homework_4::{config::hw4_config::HW4Parameters, questions::question_0::question_0};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

const CONFIG_PATH: &'static str = "homework-4\\config\\homework_4.toml";

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng: ThreadRng = rand::rng();
    let HW4Parameters {
        question_0_parameters_path,
    } = toml::from_str(read_to_string(CONFIG_PATH.to_string())?.as_str())?;

    question_0(question_0_parameters_path)?;
    // question_1(question_1_parameters_path, &mut rng)?;
    // question_2(question_2_parameters_path, &mut rng)?;
    // question_3(question_3_parameters_path, &mut rng)?;
    // question_4(question_4_parameters_path)?;

    println!("Finished!");
    return Ok(());
}
