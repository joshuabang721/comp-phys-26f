use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HW4Parameters {
    pub question_1_parameters_path: String,
    pub question_2_parameters_path: String,
}

pub trait Randomize {
    type Params;
    fn random_initialize(params: Self::Params, rng: &mut ThreadRng) -> Self;
}
