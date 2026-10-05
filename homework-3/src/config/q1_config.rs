use crate::config::hw3_config::RandStepDistType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1Parameters {
    pub simulation_parameters: Q1SimParameters,
    pub program_parameters: Q1PgmParameters,
    pub distribution_parameters: Q1DstParameters,
    pub figures_parameters: Q1FigParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1SimParameters {
    pub number_of_walks: usize,
    pub number_of_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q1PgmParameters {
    pub should_generate_statistics: bool,
    pub should_generate_figures: bool,
    pub mean_data_path: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Q1DstParameters {
    pub initial_position: f64,
    pub step_length: f64,
    pub random_step_dist_type: RandStepDistType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Q1FigParameters {
    pub number_of_times: usize,
    pub histogram_figure_path: String,
    pub chart_param_path: String,
}
