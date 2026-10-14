use config::config_hw::MainParameters;
use dataio::dataio::{FileDataType, read_data_from_file};
use homework::{
    homework1::homework1::homework1, homework2::homework2::homework2,
    homework3::homework3::homework3, homework4::homework4::homework4,
    homework5::homework5::homework5, homework6::homework6::homework6,
    homework7::homework7::homework7,
};
use rand::rngs::ThreadRng;

const PARAMETERS_PATH: &str = "params\\params_hw.toml";

fn main() -> () {
    let mut rng: ThreadRng = rand::rng();

    let MainParameters {
        should_run_hw: run,
        params_paths_hw,
    } = read_data_from_file(PARAMETERS_PATH.to_string(), FileDataType::Toml);

    let run_homeworks: [(bool, fn(String, &mut ThreadRng)); 7] = [
        (run.should_run_hw1, homework1),
        (run.should_run_hw2, homework2),
        (run.should_run_hw3, homework3),
        (run.should_run_hw4, homework4),
        (run.should_run_hw5, homework5),
        (run.should_run_hw6, homework6),
        (run.should_run_hw7, homework7),
    ];

    for (i, (run_hw, homework)) in run_homeworks.iter().enumerate() {
        if *run_hw {
            homework(params_paths_hw.replace("i", &(i + 1).to_string()), &mut rng);
        }
    }
    println!("Finished!");
}
