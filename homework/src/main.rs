use config::config_hw::MainParameters;
use dataio::dataio::{FileDataType, read_data_from_file};
use questions::{
    homework1::homework1::homework_1, homework2::homework2::homework_2,
    homework3::homework3::homework_3, homework4::homework4::homework_4,
    homework5::homework5::homework_5, homework6::homework6::homework_6,
    homework7::homework7::homework_7,
};

const PARAMETERS_PATH: &str = "params\\params_hw.toml";

fn main() -> () {
    let MainParameters {
        should_run_homeworks,
    } = read_data_from_file(PARAMETERS_PATH.to_string(), FileDataType::Toml);

    if should_run_homeworks[0] {
        homework_1();
    }
    if should_run_homeworks[1] {
        homework_2();
    }
    if should_run_homeworks[2] {
        homework_3();
    }
    if should_run_homeworks[3] {
        homework_4();
    }
    if should_run_homeworks[4] {
        homework_5();
    }
    if should_run_homeworks[5] {
        homework_6();
    }
    if should_run_homeworks[6] {
        homework_7();
    }

    println!("Finished!");
}
