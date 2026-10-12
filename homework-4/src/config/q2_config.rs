use crate::config::hw4_config::Randomize;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2Parameters {
    pub program_parameters: Q2PgmParameters,
    pub simulation_parameters: Q2SimParameters,
    pub distribution_parameters: Q2DstParameters,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2PgmParameters {
    pub should_calculate_statistics: bool,
    pub heat_map_data_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2SimParameters {
    pub initial_equilibration_portion: usize,
    pub number_of_steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Q2DstParameters {
    pub initial_position: (f64, f64),
    pub characteristic_step_size: f64,
    pub box_width: f64,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Q2ParticlePosition {
    pub x1: f64,
    pub x2: f64,
}

impl Add for Q2ParticlePosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x1: self.x1 + rhs.x1,
            x2: self.x2 + rhs.x2,
        }
    }
}

impl Randomize for Q2ParticlePosition {
    type Params = f64;
    fn random_initialize(params: Self::Params, rng: &mut rand::prelude::ThreadRng) -> Self {
        Self {
            x1: rng.random_range(-params..params),
            x2: rng.random_range(-params..params),
        }
    }
}
