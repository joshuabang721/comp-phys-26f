use crate::config::hw3_config::RandStepDistType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q3Parameters {
    pub simulation_parameters: Q3SimParameters,
    pub program_parameters: Q3PgmParameters,
    pub distribution_parameters: Q3DstParameters,
    pub figures_parameters: Q3FigParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q3SimParameters {
    pub number_of_walks: usize,
    pub number_of_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q3PgmParameters {
    pub should_generate_statistics: bool,
    pub should_generate_figures: bool,
    pub step_data_path: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Q3DstParameters {
    pub initial_position: f64,
    pub step_length_min: f64,
    pub step_length_max: f64,
    pub step_length_num: usize,
    pub random_step_dist_type: RandStepDistType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Q3FigParameters {
    pub steps_figure_path: String,
    pub chart_param_path: String,
}
