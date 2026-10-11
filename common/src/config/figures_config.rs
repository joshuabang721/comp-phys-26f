use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChartParameters {
    pub chart_color: (u8, u8, u8, f64),
    pub chart_padding: f64,
    pub font: String,

    pub margin: i32,
    pub x_label_area_size: i32,
    pub y_label_area_size: i32,
    pub caption: String,
    pub caption_size: i32,

    pub max_light_lines: usize,
    pub label_size: i32,
    pub x_desc: String,
    pub y_desc: String,
    pub desc_size: i32,
}
