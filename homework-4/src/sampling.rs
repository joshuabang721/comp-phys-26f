use crate::config::hw4_config::Randomize;
use rand::{RngExt, rngs::ThreadRng};

pub fn sample_markov_chain<E, P>(n_steps: usize, params: P, rng: &mut ThreadRng) -> Vec<(E, f64)>
where
    E: Randomize<Params = P>,
    P: Copy,
{
    (0..n_steps)
        .map(|_| (E::random_initialize(params, rng), rng.random()))
        .collect()
}
