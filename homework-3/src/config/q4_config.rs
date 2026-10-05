use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q4Parameters {
    pub program_parameters: Q4PgmParameters,
    pub figures_parameters: Q4FigParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q4PgmParameters {
    pub should_generate_figures: bool,
    pub mean_param_path: String,
    pub mean_data_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q4FigParameters {
    pub num_points: usize,
    pub mixed_figure_path: String,
    pub chart_param_path: String,
}
