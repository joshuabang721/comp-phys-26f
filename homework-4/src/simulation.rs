pub fn simulate_markov_chain<T: Fn(f64) -> f64>(
    x0: f64,
    samples: Vec<(f64, f64)>,
    pdf: T,
    init_eq_portion: usize,
) -> Vec<f64> {
    let mut x = 0.0;
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
