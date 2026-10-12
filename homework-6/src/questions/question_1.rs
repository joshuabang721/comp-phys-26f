use common::data_io::write_data_to_json;
use rand::{RngExt, rngs::ThreadRng};
use std::error::Error;

pub fn question_1(rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let n: usize = 400;
    let p: usize = 40;
    let inv_n = 1.0 / (n as f64);

    // x[mu][i]
    let ising_spins: Vec<Vec<i32>> = (0..p)
        .map(|_| {
            (0..n)
                .map(|_| if rng.random_bool(0.5) { 1 } else { -1 })
                .collect()
        })
        .collect();
    write_data_to_json(
        "homework-6\\output\\question_1\\ising_spin.json".to_string(),
        &ising_spins,
    )?;

    let interaction_matrix: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        0.0
                    } else {
                        inv_n
                            * (0..p)
                                .map(|mu| ising_spins[mu][i] * ising_spins[mu][j])
                                .sum::<i32>() as f64
                    }
                })
                .collect()
        })
        .collect();
    write_data_to_json(
        "homework-6\\output\\question_1\\interaction_matrix.json".to_string(),
        &interaction_matrix,
    )?;

    Ok(())
}
