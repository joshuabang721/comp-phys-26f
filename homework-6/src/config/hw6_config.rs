use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HW6Parameters {
    pub question_1_parameters_path: String,
    pub question_2_parameters_path: String,
    pub question_3_parameters_path: String,
    pub question_4_parameters_path: String,
    pub question_5_parameters_path: String,
    pub question_6_parameters_path: String,
}
