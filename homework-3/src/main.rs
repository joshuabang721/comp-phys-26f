use homework_3::{
    config::hw3_config::HW3Parameters,
    questions::{
        question_1::question_1, question_2::question_2, question_3::question_3,
        question_4::question_4,
    },
};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

const CONFIG_PATH: &'static str = "homework-3\\config\\homework_3.toml";

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng: ThreadRng = rand::rng();
    let HW3Parameters {
        question_1_parameters_path,
        question_2_parameters_path,
        question_3_parameters_path,
        question_4_parameters_path,
        ..
    } = toml::from_str(read_to_string(CONFIG_PATH.to_string())?.as_str())?;

    question_1(question_1_parameters_path, &mut rng)?;
    question_2(question_2_parameters_path, &mut rng)?;
    question_3(question_3_parameters_path, &mut rng)?;
    question_4(question_4_parameters_path)?;

    println!("Finished!");
    return Ok(());
}
