use rand::{RngExt, rngs::ThreadRng};
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
                .copied()
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let icnov16_duration = icnov16_start.elapsed();
    println!(
        "In class Nov 16 finish: Time elapsed: {:?}",
        icnov16_duration
    );
}
