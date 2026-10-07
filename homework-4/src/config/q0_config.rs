use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Q0Parameters {
    pub coeff_value_path: String,

    pub lower_bound: f64,
    pub upper_bound: f64,
    pub num_steps: usize,
}
