use dataio::dataio::publish_matlab_script;
use rand::rngs::ThreadRng;
use std::time::Instant;

const PATHS: [&str; 2] = [
    "output\\output_hw9\\time.json",
    "output\\output_hw9\\recurrence.json",
];

pub fn homework6(rng: &mut ThreadRng) -> () {
    let hw6_start = Instant::now();
    println!("Homework 6 start!");

    let run_hw6question1 = true;
    let run_hw6question2 = true;
    let run_hw6matlab = true;

    if run_hw6question1 {
        hw6_question1(rng);
    }
    // if run_hw6question2 {
    //     hw6_question2();
    // }
    if run_hw6matlab {
        publish_matlab_script(
            "scripts\\matlab_hw6.m",
            "matlab_hw6",
            "output\\output_hw6\\hw6_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw6_duration = hw6_start.elapsed();
    println!("Homework 6 finish! Time elapsed: {:?}", hw6_duration);
}

fn hw6_question1(_rng: &mut ThreadRng) -> () {
    let j_i_j = hebbian_interaction_j_i_j;
    let e = energy_e;
    let h_i = local_field_h_i;
    let p_i = glauber_update_p_i;
    let m1 = retrieval_overlap_m1;
    let avgmbar = late_time_average_avgmbar;
}

fn hebbian_interaction_j_i_j(memory_x_mu_i: Vec<Vec<f64>>, n: usize) -> Vec<Vec<f64>> {
    let x_mu_i = memory_x_mu_i;
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        0.0
                    } else {
                        x_mu_i.iter().map(|x_i| x_i[i] * x_i[j]).sum::<f64>() / (n as f64)
                    }
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

fn energy_e(interaction_j_i_k: Vec<Vec<f64>>, neurons_s_i: Vec<i32>) -> f64 {
    let j_i_k = interaction_j_i_k;
    let s_i = neurons_s_i;
    -0.5 * j_i_k
        .iter()
        .enumerate()
        .map(|(i, j_k)| {
            j_k.iter()
                .enumerate()
                .map(|(k, j)| j * (s_i[i] as f64) * (s_i[k] as f64))
                .sum::<f64>()
        })
        .sum::<f64>()
}

fn local_field_h_i(interaction_j_i_k: Vec<Vec<f64>>, neurons_s_i: Vec<i32>) -> Vec<f64> {
    let j_i_k = interaction_j_i_k;
    let s_i = neurons_s_i;
    j_i_k
        .iter()
        .map(|j_k| {
            j_k.iter()
                .enumerate()
                .map(|(k, j)| j * (s_i[k] as f64))
                .sum::<f64>()
        })
        .collect::<Vec<f64>>()
}

fn glauber_update_p_i(local_field_h_i: Vec<f64>, temperature_t: f64) -> Vec<f64> {
    let h_i = local_field_h_i;
    let t = temperature_t;
    h_i.iter()
        .map(|h| 1.0 / (1.0 + (-2.0 * h / t).exp()))
        .collect::<Vec<f64>>()
}

fn retrieval_overlap_m1(memory_x_1_i: Vec<f64>, neurons_s_i: Vec<i32>) -> f64 {
    let iter = std::iter::zip(memory_x_1_i, neurons_s_i);
    let n = iter.len();
    iter.map(|(x_1_i, s_i)| x_1_i * (s_i as f64)).sum::<f64>() / (n as f64)
}

fn late_time_average_avgmbar(
    retrieval_overlap_m1_r_t: Vec<Vec<f64>>,
    initial_transient_tb: usize,
) -> f64 {
    let m1_r_t = retrieval_overlap_m1_r_t;
    let tb = initial_transient_tb;
    let r = m1_r_t.len();

    m1_r_t
        .iter()
        .map(|m1_t| {
            let iter = m1_t.iter().skip(tb);
            let t = iter.len();
            iter.sum::<f64>() / (t as f64)
        })
        .sum::<f64>()
        / (r as f64)
}
