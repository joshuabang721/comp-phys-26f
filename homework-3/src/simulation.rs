pub fn simulate_random_walks(
    random_step_samples: Vec<Vec<Vec<f64>>>,
    x0: f64,
) -> Vec<Vec<Vec<f64>>> {
    let mut i = 0;
    random_step_samples
        .iter()
        .map(|step_samples| {
            i += 1;
            let walks: Vec<Vec<f64>> = step_samples
                .iter()
                .map(|steps| {
                    let mut xn: f64 = x0;
                    (0..=steps.len())
                        .map(|x| {
                            if x == 0 {
                                xn
                            } else {
                                xn += steps[x - 1];
                                xn
                            }
                        })
                        .collect()
                })
                .collect();
            walks
        })
        .collect()
}
