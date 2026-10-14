use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MainParameters {
    pub params_paths_hw: String,
    pub should_run_hw: HwShouldRun,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HwShouldRun {
    pub should_run_hw1: bool,
    pub should_run_hw2: bool,
    pub should_run_hw3: bool,
    pub should_run_hw4: bool,
    pub should_run_hw5: bool,
    pub should_run_hw6: bool,
    pub should_run_hw7: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgramParameters {
    pub should_calculate_statistics: bool,
    pub should_generate_figures: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChartParameters {
    pub chart_color: (u8, u8, u8, f64),
    pub chart_padding: f64,
    pub font: String,

    pub margin: i32,
    pub x_label_area_size: i32,
    pub y_label_area_size: i32,
    pub caption_size: i32,

    pub max_light_lines: usize,
    pub label_size: i32,
    pub desc_size: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FigureParameters {
    pub caption: String,
    pub x_desc: String,
    pub y_desc: String,
}
