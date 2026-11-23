use rand::{rngs::ThreadRng, RngExt};
use std::time::Instant;

pub fn inclass_nov16(rng: &mut ThreadRng) -> () {
    let icnov16_start = Instant::now();
    println!("In class Nov 16 start!");

    let n = 32;
    let m = 600;
    let p = 0.08;
    let cycle = [1, 1, -1, -1];
    // let s0_i = cycle.iter().cycle().take(n).copied().collect::<Vec<i32>>();
    let x_n_i = (0..m)
        .map(|_| {
            let r = rng.random_range(0..4);
            cycle
                .iter()
                .cycle()
                .skip(r)
                .take(n)
                .map(|&sni| if rng.random_bool(p) { -sni } else { sni })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let sbar_i = x_n_i
        .iter()
        .fold(vec![0.0; n], |mut sbar_i, xn_i| {
            sbar_i
                .iter_mut()
                .zip(xn_i)
                .for_each(|(sbar, &x)| *sbar += x as f64);
            sbar_i
        })
        .into_iter()
        .map(|s| s / (m as f64))
        .collect::<Vec<f64>>();
    let xc_n_i = x_n_i
        .into_iter()
        .map(|xn_i| {
            xn_i.iter()
                .enumerate()
                .map(|(i, &xni)| (xni as f64) - sbar_i[i])
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let icnov16_duration: std::time::Duration = icnov16_start.elapsed();
    println!(
        "In class Nov 16 finish: Time elapsed: {:?}",
        icnov16_duration
    );
}
