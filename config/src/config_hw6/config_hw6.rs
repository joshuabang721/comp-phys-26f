use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Hw6Parameters {
    pub should_run_questions: [bool; 6],
}
