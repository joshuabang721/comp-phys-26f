use config::config_hw6::config_hw6q1::Hw6Q1Parameters;
use dataio::dataio::{FileDataType, read_data_from_file, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};

pub fn question_1(params_path: String, rng: &mut ThreadRng) -> () {
    let Hw6Q1Parameters {
        number_of_neurons: n,
        number_of_pattern: p,
        ising_spins_data_path,
    } = read_data_from_file(params_path, FileDataType::Toml);

    let ising_spins: Vec<Vec<i32>>;
    ising_spins = (0..p)
        .map(|_| {
            (0..n)
                .map(|_| if rng.random_bool(0.5) { 1 } else { -1 })
                .collect()
        })
        .collect();
    write_data_to_file(&ising_spins, ising_spins_data_path, FileDataType::Json);
}
