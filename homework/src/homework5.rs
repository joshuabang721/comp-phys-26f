use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::{array, time::Instant};

const PATHS: [&str; 3] = [
    "output\\output_hw5\\transition_graph.json",
    "output\\output_hw5\\transition_matrix_1b.json",
    "output\\output_hw5\\transition_matrix_1c.json",
];

pub fn homework5(rng: &mut ThreadRng) -> () {
    let hw5_start = Instant::now();
    println!("Homework 5 start!");

    // hw5_question1();
    // hw5_question2();
    // hw5_question3();

    publish_matlab_script(
        "scripts\\matlab_hw5.m",
        "matlab_hw5",
        "output\\output_hw5\\hw5_document.pdf",
        PATHS.to_vec(),
    );

    let hw5_duration = hw5_start.elapsed();
    println!("Homework 5 finish! Time elapsed: {:?}", hw5_duration);
}

fn hw5_question1() -> () {
    let hw5q1_start = Instant::now();
    println!("Hw 5 question 1 start!");

    hw5_question1::hw5q1_parta();
    hw5_question1::hw5q1_partb();
    hw5_question1::hw5q1_partc();

    let hw5q1_duration = hw5q1_start.elapsed();
    println!("Hw 5 question 1 finish! Time elapsed: {:?}", hw5q1_duration);
}

mod hw5_question1 {
    use crate::homework5::{PATHS, transition_graph, transition_matrix};
    use dataio::dataio::{FileDataType, write_data_to_file};

    pub(super) fn hw5q1_parta() -> () {
        let beta = 1.0;
        let transition_graph = transition_graph(beta).iter().fold(
            (
                Vec::with_capacity(32),
                Vec::with_capacity(32),
                Vec::with_capacity(32),
            ),
            |mut acc, x| {
                if x.2.abs() > 1e-15 {
                    acc.0.push(x.0);
                    acc.1.push(x.1);
                    acc.2.push(x.2);
                }
                acc
            },
        );

        write_data_to_file(&transition_graph, PATHS[0], FileDataType::Json);
    }

    pub(super) fn hw5q1_partb() -> () {
        let beta = [10.0, 1.0, 0.1];
        let transition_matrix = beta.map(|b| transition_matrix(b));

        write_data_to_file(&transition_matrix, PATHS[1], FileDataType::Json);
    }

    pub(super) fn hw5q1_partc() -> () {
        let dtemp = 0.1;
        let mut temp = 0.0;
        let betas = (1..=100)
            .map(|_| {
                temp += dtemp;
                1.0 / temp
            })
            .collect::<Vec<f64>>();

        let transition_matrices = betas
            .iter()
            .map(|&b| transition_matrix(b))
            .collect::<Vec<[[f64; 8]; 8]>>();

        write_data_to_file(&transition_matrices, PATHS[2], FileDataType::Json);
    }
}

fn hw5_question2() -> () {
    let hw5q2_start = Instant::now();
    println!("Hw 5 question 2 start!");

    let hw5q2_duration = hw5q2_start.elapsed();
    println!("Hw 5 question 2 finish! Time elapsed: {:?}", hw5q2_duration);
}

fn hw5_question3() -> () {
    let hw5q3_start = Instant::now();
    println!("Hw 5 question 3 start!");

    let hw5q3_duration = hw5q3_start.elapsed();
    println!("Hw 5 question 3 finish! Time elapsed: {:?}", hw5q3_duration);
}

fn transition_graph(beta: f64) -> Vec<(usize, usize, f64)> {
    (0..8)
        .flat_map(|x_idx| {
            let x: [i32; 3] = array::from_fn(|i| if ((x_idx >> i) & 1) == 1 { 1 } else { -1 });
            let mut p_inv = 1.0;
            (0..=3).map(move |s| {
                if s < 3 {
                    let y_idx = x_idx ^ (1 << s);
                    let y: [i32; 3] =
                        array::from_fn(|i| if ((y_idx >> i) & 1) == 1 { 1 } else { -1 });
                    let p = transition_probability(x, y, beta) / 3.0;
                    p_inv -= p;
                    (x_idx, y_idx, p)
                } else {
                    (x_idx, x_idx, p_inv)
                }
            })
        })
        .collect()
    //
}

fn transition_matrix(beta: f64) -> [[f64; 8]; 8] {
    transition_graph(beta)
        .iter()
        .fold([[0.0; 8]; 8], |mut acc, (x, y, p)| {
            acc[*y][*x] += p;
            acc
        })
}

fn transition_probability(x: [i32; 3], y: [i32; 3], beta: f64) -> f64 {
    let e = configuration_energy;
    (-beta * ((e(y) - e(x)) as f64)).exp().min(1.0)
}

fn configuration_energy(x: [i32; 3]) -> i32 {
    -(x[0] * x[1] + x[1] * x[2] + x[2] * x[0])
}
