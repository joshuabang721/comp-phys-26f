use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Hw6Q1Parameters {
    pub number_of_neurons: usize,
    pub number_of_pattern: usize,
    pub ising_spins_data_path: String,
}
