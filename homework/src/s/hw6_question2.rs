use std::collections::HashSet;

use dataio::dataio::{FileDataType, read_data_from_file, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};

use crate::homework6::hw6_question1::calculate_local_field;

pub fn hw6_question2(_params_path: &str, rng: &mut ThreadRng) -> () {
    let n = 400;
    // let p = 40;
    let t = 40;

    //  x[mu][i]
    let memory: Vec<Vec<i32>> = read_data_from_file(
        "output\\output_hw6\\memory_patterns.json",
        FileDataType::Json,
    );

    //  J[i][j]
    let interaction: Vec<Vec<f64>> = read_data_from_file(
        "output\\output_hw6\\interaction_matrix.json",
        FileDataType::Json,
    );

    let temperature = [0.1, 0.5, 1.0];
    let f = 0.05;

    //  s[T][t][i]
    let spins: Vec<Vec<Vec<i32>>> = temperature
        .iter()
        .map(|&temp| {
            let mut sigma_t: Vec<i32> = Vec::new();
            let mut prob_t: Vec<f64> = Vec::new();
            (0..=t)
                .map(|t| {
                    sigma_t = if t == 0 {
                        let m = (f * (n as f64)) as usize;
                        let mut flips: HashSet<usize> = HashSet::with_capacity(m);
                        while flips.len() < m {
                            flips.insert(rng.random_range(0..n) as usize);
                        }
                        let mut x_init = memory[0].clone();
                        for idx in flips {
                            x_init[idx] = -x_init[idx];
                        }
                        x_init
                    } else {
                        sigma_t
                            .iter()
                            .enumerate()
                            .map(|(t, &s_ti)| {
                                if rng.random::<f64>() < prob_t[t] {
                                    s_ti
                                } else {
                                    -s_ti
                                }
                            })
                            .collect()
                    };
                    prob_t = calculate_local_field(&sigma_t, &interaction)
                        .iter()
                        .map(|&h| 1.0 / (1.0 + (-2.0f64 * h / temp).exp()))
                        .collect();
                    sigma_t.clone()
                })
                .collect()
        })
        .collect();

    write_data_to_file(
        &spins,
        "output\\output_hw6\\output_hw6q2\\q2_spins.json",
        FileDataType::Json,
    );

    //  m[mu][t]
    let overlap: Vec<Vec<Vec<f64>>> = spins
        .iter()
        .map(|s_t| {
            s_t.iter()
                .map(|s_tt| {
                    memory
                        .iter()
                        .map(|x_m| {
                            1.0 / (n as f64)
                                * (0..(s_tt.len().min(x_m.len())))
                                    .map(|i| s_tt[i] * x_m[i])
                                    .sum::<i32>() as f64
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    write_data_to_file(
        &overlap,
        "output\\output_hw6\\output_hw6q2\\q2_overlap.json",
        FileDataType::Json,
    );
}
