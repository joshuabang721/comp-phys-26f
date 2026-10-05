use crate::config::hw3_config::RandStepDistType;
use rand::{RngExt, rngs::ThreadRng};
use std::f64::consts::PI;

pub fn sample_random_steps(
    n_walks: usize,
    n_steps: usize,
    step_lengths: &Vec<f64>,
    dist_type: RandStepDistType,
    rng: &mut ThreadRng,
) -> Vec<Vec<Vec<f64>>> {
    let random_steps: Vec<Vec<Vec<f64>>> = step_lengths
        .iter()
        .map(|a| {
            let steps_pdf: Box<dyn Fn(&mut ThreadRng) -> f64> = random_steps_pdf(&dist_type, *a);
            (0..n_walks)
                .map(|_| (0..n_steps).map(|_| steps_pdf(rng)).collect())
                .collect()
        })
        .collect();
    random_steps
}

fn random_steps_pdf(pdf_type: &RandStepDistType, a: f64) -> Box<dyn Fn(&mut ThreadRng) -> f64> {
    if a == 0.0 {
        return Box::new(|_| 0.0);
    }

    match pdf_type {
        RandStepDistType::TwoSidedExp => Box::new(move |rng: &mut ThreadRng| {
            let x: f64 = rng.random();
            if x < 0.5 {
                a * (2.0 * x).ln()
            } else {
                -a * (-2.0 * x + 2.0).ln()
            }
        }),
        RandStepDistType::SemiCircle => Box::new(move |rng: &mut ThreadRng| {
            let k: f64 = PI * PI * a * a * a * a / 4.0;
            let a2: f64 = a * a;
            let ymax: f64 = 2.0 / (PI * a);
            let mut x: f64 = 2.0 * a;
            let mut y: f64 = a;
            while x * x + k * y * y > a2 {
                x = rng.random_range(-a..=a);
                y = rng.random_range(0.0..=ymax);
            }
            x
        }),
    }
}
