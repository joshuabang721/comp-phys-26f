use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1Parameters {
    pub coefficient_parameters: Q1CofParameters,
    pub program_parameters: Q1PgmParameters,
    pub simulation_parameters: Q1SimParameters,
    pub distribution_parameters: Q1DstParameters,
    pub figure_parameters: Q1FigParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1CofParameters {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub num_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1PgmParameters {
    pub should_calculate_coefficient: bool,
    pub should_calculate_statistics: bool,
    pub should_generate_figures: bool,

    pub coefficient_value_path: String,
    pub markov_chain_data_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1SimParameters {
    pub initial_equilibration_portion: usize,
    pub number_of_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1DstParameters {
    pub initial_position: f64,
    pub characteristic_step_size: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1FigParameters {
    pub figure_path: String,
    pub chart_parameters_path: String,
}
