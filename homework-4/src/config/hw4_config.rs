use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HW4Parameters {
    pub question_0_parameters_path: String,
}

pub fn target_probability_distribution(c: f64) -> impl Fn(f64) -> f64 {
    move |x| c * (-4.0 * (x * x - 1.0) * (x * x - 1.0))
}
