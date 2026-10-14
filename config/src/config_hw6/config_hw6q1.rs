use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Hw6Q1Parameters {
    pub number_of_neurons: usize,
    pub number_of_pattern: usize,
    pub hw6q1_paths: Hw6Q1Paths,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Hw6Q1Paths {
    pub memory_patterns_data_path: String,
    pub interaction_matrix_data_path: String,
    pub ising_spins_data_path: String,
    pub overlap_path: String,
}
