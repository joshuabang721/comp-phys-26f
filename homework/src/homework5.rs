use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::{array, time::Instant};

pub fn homework5(rng: &mut ThreadRng) -> () {
    let hw5_start = Instant::now();
    println!("Homework 5 start!");

    hw5_question1();
    hw5_question2();
    hw5_question3();

    publish_matlab_script(
        "scripts\\matlab_hw5.m",
        "matlab_hw5",
        "output\\output_hw5\\hw5_document.pdf",
        vec![
            "output\\output_hw5\\transition_graph.json",
            "output\\output_hw5\\transition_matrix.json",
        ],
    );

    let hw5_duration = hw5_start.elapsed();
    println!("Homework 5 finish! Time elapsed: {:?}", hw5_duration);
}

fn hw5_question1() -> () {
    let hw5q1_start = Instant::now();
    println!("Hw 5 question 1 start!");

    let beta = 1.0;
    let transition_graph = transition_graph(beta);

    write_data_to_file(
        &transition_graph,
        "output\\output_hw5\\transition_graph.json",
        FileDataType::Json,
    );

    let beta = [10.0, 1.0, 0.1];
    let transition_matrix = beta.map(|b| transition_matrix(b));

    write_data_to_file(
        &transition_matrix,
        "output\\output_hw5\\transition_matrix.json",
        FileDataType::Json,
    );

    let hw5q1_duration = hw5q1_start.elapsed();
    println!("Hw 5 question 1 finish! Time elapsed: {:?}", hw5q1_duration);
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

fn transition_graph(beta: f64) -> (Vec<usize>, Vec<usize>, Vec<f64>) {
    (0..8)
        .flat_map(|k| {
            let configuration: [i32; 3] =
                array::from_fn(|i| if ((k >> i) & 1) == 0 { 1 } else { -1 });
            let mut p_inv = 1.0;
            (0..=3).map(move |s| {
                if s < 3 {
                    let x: [i32; 3] = configuration;
                    let y: [i32; 3] = array::from_fn(|i| {
                        if i == s {
                            -configuration[i]
                        } else {
                            configuration[i]
                        }
                    });
                    let p = transition_probability(x, y, beta) / 3.0;
                    p_inv -= p;
                    let x_idx = x
                        .iter()
                        .enumerate()
                        .map(|(i, s)| (if *s == -1 { 1 } else { 0 }) << i)
                        .sum();
                    let y_idx = y
                        .iter()
                        .enumerate()
                        .map(|(i, s)| (if *s == -1 { 1 } else { 0 }) << i)
                        .sum();
                    (x_idx, y_idx, p)
                } else {
                    (k, k, p_inv)
                }
            })
        })
        .fold(
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
        )
}

fn transition_matrix(beta: f64) -> [[f64; 8]; 8] {
    array::from_fn(|y_idx| {
        let y: [i32; 3] = array::from_fn(|i| if ((y_idx >> i) & 1) == 0 { 1 } else { -1 });
        array::from_fn(|x_idx| {
            let x: [i32; 3] = array::from_fn(|i| if ((x_idx >> i) & 1) == 0 { 1 } else { -1 });
            transition_probability(x, y, beta)
        })
    })
}

fn transition_probability(x: [i32; 3], y: [i32; 3], beta: f64) -> f64 {
    let e = configuration_energy;
    (-beta * ((e(y) - e(x)) as f64)).exp().min(1.0)
}

fn configuration_energy(x: [i32; 3]) -> i32 {
    -(x[0] * x[1] + x[1] * x[2] + x[2] * x[0])
}
