use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HW3Parameters {
    pub question_1_parameters_path: String,
    pub question_2_parameters_path: String,
    pub question_3_parameters_path: String,
    pub question_4_parameters_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RandStepDistType {
    TwoSidedExp,
    SemiCircle,
}
