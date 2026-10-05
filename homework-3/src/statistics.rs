use std::f64::consts::PI;

use crate::config::hw3_config::RandStepDistType;

pub fn q1_statistic_walk_dists(walks: &Vec<Vec<f64>>, n_times: usize) -> Vec<(usize, Vec<f64>)> {
    let n_steps = walks[0].len() - 1;
    let dt = n_steps / n_times;
    (1..=n_times)
        .map(|i| {
            let n = if i == n_times { n_steps } else { i * dt };
            (n, walks.iter().map(|walk| walk[n]).collect())
        })
        .collect()
}

pub fn q2_statistic_walk_sigma(walks: &Vec<Vec<f64>>) -> Vec<f64> {
    let n_steps = walks[0].len();
    let mut mean: Vec<f64> = vec![0.0; n_steps];
    let mut sigma: Vec<f64> = vec![0.0; n_steps];
    let c = 1.0 / (walks.len() as f64);
    for walk in walks {
        for n in 0..n_steps {
            mean[n] += walk[n];
            sigma[n] += walk[n] * walk[n];
        }
    }
    for n in 0..n_steps {
        mean[n] *= c;
        sigma[n] = (c * sigma[n] - mean[n] * mean[n]).sqrt();
    }
    sigma
}

pub fn q3_statistic_step_sigma(walks: Vec<Vec<Vec<f64>>>) -> Vec<f64> {
    let mut sigmas: Vec<f64> = Vec::with_capacity(walks.len());
    for a in walks {
        let mut sigma = 0.0;
        let n_walk = a.len();
        for w in a {
            let mut m = 0.0;
            let mut s = 0.0;
            let n_steps = w.len();
            for st in w {
                m += st;
                s += st * st;
            }
            m /= n_steps as f64;
            sigma += (s / (n_steps as f64) - m * m).sqrt();
        }
        sigmas.push(sigma / (n_walk as f64));
    }
    sigmas
}

pub fn q4_statistic_anal_soln(
    initial_position: f64,
    step_length: f64,
    random_step_dist_type: RandStepDistType,
) -> impl Fn(f64, f64) -> f64 {
    let k = match random_step_dist_type {
        RandStepDistType::TwoSidedExp => 2.0,
        RandStepDistType::SemiCircle => 0.25,
    };
    let d = k * step_length * step_length / (2.0 * 1.0); //  dt = 1
    move |n: f64, x: f64| {
        1.0 / (4.0 * PI * d * n).sqrt()
            * (-(x - initial_position) * (x - initial_position) / (4.0 * d * n)).exp()
    }
}
