use crate::config::hw3_config::RandStepDistType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2Parameters {
    pub simulation_parameters: Q2SimParameters,
    pub program_parameters: Q2PgmParameters,
    pub distribution_parameters: Q2DstParameters,
    pub figures_parameters: Q2FigParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2SimParameters {
    pub number_of_walks: usize,
    pub number_of_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2PgmParameters {
    pub should_generate_statistics: bool,
    pub should_generate_figures: bool,
    pub sigma_data_path: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Q2DstParameters {
    pub initial_position: f64,
    pub step_length: f64,
    pub random_step_dist_type: RandStepDistType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Q2FigParameters {
    pub histogram_figure_path: String,
    pub chart_param_path: String,
}
