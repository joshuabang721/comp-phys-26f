use std::{fmt, ops::Add};

pub fn simulate_markov_chain<E, F>(
    pdf: F,
    x0: E,
    samples: Vec<(E, f64)>,
    init_eq_portion: usize,
) -> Vec<E>
where
    E: Add<Output = E> + Copy + fmt::Debug + Default,
    F: Fn(E) -> f64,
{
    let mut x: E = E::default();
    let mut px = 0.0;
    (0..=samples.len())
        .map(|i| {
            if i == 0 {
                x = x0;
                px = pdf(x);
            } else {
                let sample = samples[i - 1];
                let y = x + sample.0;
                let py = pdf(y);
                let r = (py / px).min(1.0);
                if sample.1 < r {
                    x = y;
                    px = py;
                }
            }
            x
        })
        .skip(init_eq_portion)
        .collect()
}
