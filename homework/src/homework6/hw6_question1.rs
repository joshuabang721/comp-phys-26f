use std::collections::HashSet;

use config::config_hw6::config_hw6q1::Hw6Q1Parameters;
use dataio::dataio::{FileDataType, read_data_from_file, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};

pub fn hw6_question1(params_path: String, rng: &mut ThreadRng) -> () {
    let Hw6Q1Parameters {
        number_of_neurons: n,
        number_of_pattern: p,
        memory_patterns_data_path,
        interaction_matrix_data_path,
        ising_spins_data_path,
        overlap_path,
    } = read_data_from_file(params_path, FileDataType::Toml);

    // x[m][i]
    let memory_patterns: Vec<Vec<i32>> = (0..p)
        .map(|_| {
            (0..n)
                .map(|_| if rng.random_bool(0.5) { 1 } else { -1 })
                .collect()
        })
        .collect();
    write_data_to_file(
        &memory_patterns,
        memory_patterns_data_path,
        FileDataType::Json,
    );

    // J[i][j]
    let interaction_matrix = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        0.0
                    } else {
                        1.0 / (n as f64)
                            * (0..p)
                                .map(|mu| memory_patterns[mu][i] * memory_patterns[mu][j])
                                .sum::<i32>() as f64
                    }
                })
                .collect()
        })
        .collect();
    write_data_to_file(
        &interaction_matrix,
        interaction_matrix_data_path,
        FileDataType::Json,
    );

    let initial_error_fraction = vec![0.05, 0.25, 0.40];
    //  sigma[f][t][i]
    let mut s_t = Vec::new();
    let ising_spins: Vec<Vec<Vec<i32>>> = initial_error_fraction
        .iter()
        .map(|f| {
            (0..=30)
                .map(|t| {
                    s_t = if t == 0 {
                        let m = (f * (n as f64)) as usize;
                        let mut flips: HashSet<usize> = HashSet::with_capacity(m);
                        while flips.len() < m {
                            flips.insert(rng.random_range(0..n) as usize);
                        }
                        let mut x_init = memory_patterns[0].clone();
                        for idx in flips {
                            x_init[idx] = -x_init[idx];
                        }
                        x_init
                    } else {
                        synchronous_update(&s_t, &interaction_matrix)
                    };
                    s_t.clone()
                })
                .collect()
        })
        .collect();
    write_data_to_file(&ising_spins, ising_spins_data_path, FileDataType::Json);

    //  m
    let overlap: Vec<Vec<Vec<f64>>> = initial_error_fraction
        .iter()
        .enumerate()
        .map(|(f, _)| {
            (0..p)
                .map(|mu| {
                    (0..=30)
                        .map(|t| {
                            (0..n)
                                .map(|i| {
                                    1.0 / (n as f64)
                                        * (memory_patterns[mu][i] * ising_spins[f][t][i]) as f64
                                })
                                .sum()
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    write_data_to_file(&overlap, overlap_path, FileDataType::Json);
}

/// h[i]
fn calculate_local_field(sigma: &Vec<i32>, interaction_matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    (0..interaction_matrix.len())
        .map(|i| {
            (0..sigma.len())
                .map(|k| interaction_matrix[i][k] * (sigma[k] as f64))
                .sum()
        })
        .into_iter()
        .collect()
}

fn synchronous_update(sigma: &Vec<i32>, interaction_matrix: &Vec<Vec<f64>>) -> Vec<i32> {
    let h = calculate_local_field(sigma, interaction_matrix);
    (0..sigma.len())
        .map(|i| match h[i] {
            x if x > 0.0 => 1,
            x if x < 0.0 => -1,
            _ => sigma[i],
        })
        .collect()
}
