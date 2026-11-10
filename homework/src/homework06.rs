use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng, seq::SliceRandom};
use std::time::Instant;

const PATHS: [&str; 8] = [
    "output\\output_hw06\\plot1_params.json",
    "output\\output_hw06\\plot1_alpha.json",
    "output\\output_hw06\\plot1_freq.json",
    "output\\output_hw06\\plot1_data.json",
    "output\\output_hw06\\plot2_params.json",
    "output\\output_hw06\\plot2_alpha.json",
    "output\\output_hw06\\plot2_temp.json",
    "output\\output_hw06\\plot2_data.json",
];

pub fn homework6(rng: &mut ThreadRng) -> () {
    let hw06_start = Instant::now();
    println!("Homework 6 start!");

    let run_hw06question1 = true;
    let run_hw06question2 = false;
    let run_hw06matlab = true;

    if run_hw06question1 {
        hw06_question1(rng);
    }
    if run_hw06question2 {
        hw06_question2(rng);
    }
    if run_hw06matlab {
        publish_matlab_script(
            "scripts\\matlab_hw06.m",
            "matlab_hw06",
            "output\\output_hw06\\hw06_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw06_duration = hw06_start.elapsed();
    println!("Homework 6 finish! Time elapsed: {:?}", hw06_duration);
}

fn hw06_question1(rng: &mut ThreadRng) -> () {
    let hw06q1_start = Instant::now();
    println!("Hw 6 Question 1 start!");

    let n = 100;
    let r = 50;
    let tb = 40;
    let tf = 50;

    let alpha = {
        let min = 0.0;
        let max = 0.6;
        let num = 20;
        let del = (max - min) / (num as f64);
        (0..=num)
            .map(|i| min + (i as f64) * del)
            .collect::<Vec<f64>>()
    };
    let temp = 0.01;
    let freq = {
        let min = 0.0;
        let max = 0.6;
        let num = 20;
        let del = (max - min) / (num as f64);
        (0..=num)
            .map(|i| min + (i as f64) * del)
            .collect::<Vec<f64>>()
    };

    let map = alpha
        .iter()
        .map(|&a| {
            println!("a = {}", a);

            let p = (a * (n as f64)).round() as usize;
            if p == 0 {
                vec![0.0; freq.len()]
            } else {
                let x_mu_i = (0..p)
                    .map(|_| {
                        (0..n)
                            .map(|_| if rng.random_bool(0.5) { 1 } else { -1 })
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>();
                let x0_i = x_mu_i[0].clone();
                let j_i_k = hebbian_interaction_j_i_j(x_mu_i, n);

                freq.iter()
                    .map(|&f| {
                        println!("f = {}", f);

                        let m1_r_t = (0..r)
                            .map(|_| {
                                (0..=tf)
                                    .scan(Vec::new(), |s_i, time| {
                                        if time == 0 {
                                            // println!("New Run");

                                            *s_i = x0_i.clone();
                                            let m = (f * (n as f64)).round() as usize;
                                            let mut idxs = (0..n).collect::<Vec<usize>>();
                                            idxs.shuffle(rng);
                                            for &i in idxs.iter().take(m) {
                                                s_i[i] = -s_i[i];
                                            }
                                        } else {
                                            let p_i = glauber_update_p_i(
                                                local_field_h_i(&j_i_k, &s_i),
                                                temp,
                                            );
                                            for (s, p) in s_i.iter_mut().zip(p_i) {
                                                *s = if rng.random::<f64>() < p { 1 } else { -1 };
                                            }
                                        };
                                        Some(retrieval_overlap_m1(&x0_i, &s_i))
                                    })
                                    .skip(tb)
                                    .collect::<Vec<f64>>()
                            })
                            .collect::<Vec<Vec<f64>>>();
                        late_time_average_avgmbar(m1_r_t)
                    })
                    .collect::<Vec<f64>>()
            }
        })
        .collect::<Vec<Vec<f64>>>();

    write_data_to_file(&vec![n, r, tb, tf], PATHS[0], FileDataType::Json);
    write_data_to_file(&alpha, PATHS[1], FileDataType::Json);
    write_data_to_file(&freq, PATHS[2], FileDataType::Json);
    write_data_to_file(&map, PATHS[3], FileDataType::Json);

    let hw06q1_duration = hw06q1_start.elapsed();
    println!(
        "Hw 6 Question 1 finish! Time elapsed: {:?}",
        hw06q1_duration
    );
}

fn hw06_question2(rng: &mut ThreadRng) -> () {
    let hw06q2_start = Instant::now();
    println!("Hw 6 Question 2 start!");

    let n = 100;
    let r = 50;
    let tb = 40;
    let tf = 50;

    let alpha = {
        let min = 0.0;
        let max = 0.3;
        let num = 20;
        let del = (max - min) / (num as f64);
        (0..=num)
            .map(|i| min + (i as f64) * del)
            .collect::<Vec<f64>>()
    };
    let temp = {
        let min = 0.0;
        let max = 1.0;
        let num = 20;
        let del = (max - min) / (num as f64);
        (0..=num)
            .map(|i| min + (i as f64) * del)
            .collect::<Vec<f64>>()
    };
    let freq = 0.05;

    let map = alpha
        .iter()
        .map(|&a| {
            println!("a = {}", a);

            let p = (a * (n as f64)).round() as usize;
            if p == 0 {
                vec![0.0; temp.len()]
            } else {
                let x_mu_i = (0..p)
                    .map(|_| {
                        (0..n)
                            .map(|_| if rng.random_bool(0.5) { 1 } else { -1 })
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>();
                let x0_i = x_mu_i[0].clone();
                let j_i_k = hebbian_interaction_j_i_j(x_mu_i, n);

                temp.iter()
                    .map(|&t| {
                        println!("t = {}", t);

                        let m1_r_t = (0..r)
                            .map(|_| {
                                (0..=tf)
                                    .scan(Vec::new(), |s_i, time| {
                                        if time == 0 {
                                            // println!("New Run");

                                            *s_i = x0_i.clone();
                                            let m = (freq * (n as f64)).round() as usize;
                                            let mut idxs = (0..n).collect::<Vec<usize>>();
                                            idxs.shuffle(rng);
                                            for &i in idxs.iter().take(m) {
                                                s_i[i] = -s_i[i];
                                            }
                                        } else {
                                            let p_i = glauber_update_p_i(
                                                local_field_h_i(&j_i_k, &s_i),
                                                t,
                                            );
                                            for (s, p) in s_i.iter_mut().zip(p_i) {
                                                *s = if rng.random::<f64>() < p { 1 } else { -1 };
                                            }
                                        };
                                        Some(retrieval_overlap_m1(&x0_i, &s_i))
                                    })
                                    .skip(tb)
                                    .collect::<Vec<f64>>()
                            })
                            .collect::<Vec<Vec<f64>>>();
                        late_time_average_avgmbar(m1_r_t)
                    })
                    .collect::<Vec<f64>>()
            }
        })
        .collect::<Vec<Vec<f64>>>();

    write_data_to_file(&vec![n, r, tb, tf], PATHS[4], FileDataType::Json);
    write_data_to_file(&alpha, PATHS[5], FileDataType::Json);
    write_data_to_file(&temp, PATHS[6], FileDataType::Json);
    write_data_to_file(&map, PATHS[7], FileDataType::Json);

    let hw06q2_duration = hw06q2_start.elapsed();
    println!(
        "Hw 6 Question 2 finish! Time elapsed: {:?}",
        hw06q2_duration
    );
}

fn hebbian_interaction_j_i_j(memory_x_mu_i: Vec<Vec<i32>>, n: usize) -> Vec<Vec<f64>> {
    let x_mu_i = memory_x_mu_i;
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        0.0
                    } else {
                        x_mu_i
                            .iter()
                            .map(|x_i| (x_i[i] * x_i[j]) as f64)
                            .sum::<f64>()
                            / (n as f64)
                    }
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

fn local_field_h_i(interaction_j_i_k: &[Vec<f64>], neurons_s_i: &[i32]) -> Vec<f64> {
    let j_i_k = interaction_j_i_k;
    let s_i = neurons_s_i;
    let h = j_i_k
        .iter()
        .map(|j_k| {
            j_k.iter()
                .enumerate()
                .map(|(k, &j)| j * (s_i[k] as f64))
                .sum::<f64>()
        })
        .collect::<Vec<f64>>();
    h
}

fn glauber_update_p_i(local_field_h_i: Vec<f64>, temperature_t: f64) -> Vec<f64> {
    let h_i = local_field_h_i;
    let t = temperature_t;
    h_i.iter()
        .map(|&h| 1.0 / (1.0 + (-2.0 * h / t).exp()))
        .collect::<Vec<f64>>()
}

fn retrieval_overlap_m1(memory_x_1_i: &[i32], neurons_s_i: &[i32]) -> f64 {
    let iter = std::iter::zip(memory_x_1_i, neurons_s_i);
    let n = iter.len();
    iter.map(|(&x_1_i, s_i)| (x_1_i * s_i) as f64).sum::<f64>() / (n as f64)
}

fn late_time_average_avgmbar(retrieval_overlap_m1_r_t: Vec<Vec<f64>>) -> f64 {
    let m1_r_t = retrieval_overlap_m1_r_t;

    m1_r_t
        .iter()
        .map(|m1_t| m1_t.iter().sum::<f64>() / (m1_t.len() as f64))
        .sum::<f64>()
        / (m1_r_t.len() as f64)
}
