use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Hw6Parameters {
    pub should_run_hw6: Hw6ShouldRun,
    pub params_paths_hw6: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hw6ShouldRun {
    pub should_run_hw6q1: bool,
    pub should_run_hw6q2: bool,
    pub should_run_hw6q3: bool,
    pub should_run_hw6q4: bool,
    pub should_run_hw6q5: bool,
    pub should_run_hw6q6: bool,
}
