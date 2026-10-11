use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2Parameters {
    pub program_parameters: Q2PgmParameters,
    pub simulation_parameters: Q2SimParameters,
    pub distribution_parameters: Q2DstParameters,
    pub figure_parameters: Q2FigParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2PgmParameters {
    pub should_calculate_statistics: bool,
    pub should_generate_figures: bool,

    pub markov_chain_data_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2SimParameters {
    pub initial_equilibration_portion: usize,
    pub number_of_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2DstParameters {
    pub initial_position: f64,
    pub characteristic_step_size: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2FigParameters {
    pub figure_path: String,
    pub chart_parameters_path: String,
}
