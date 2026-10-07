use rand::{RngExt, rngs::ThreadRng};

pub fn sample_markov_chain(n_steps: usize, step_size: f64, rng: &mut ThreadRng) -> Vec<(f64, f64)> {
    (0..n_steps)
        .map(|_| (rng.random_range((-step_size..step_size)), rng.random()))
        .collect()
}
