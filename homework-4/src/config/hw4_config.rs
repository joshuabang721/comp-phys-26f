use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HW4Parameters {
    pub question_1_parameters_path: String,
    pub question_2_parameters_path: String,
}
