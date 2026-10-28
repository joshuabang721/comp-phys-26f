use dataio::dataio::publish_matlab_script;
use rand::{RngExt, rngs::ThreadRng};
use std::{f64::consts::TAU, time::Instant};

const PATHS: [&str; 3] = [
    "output\\output_hw1\\data.json",
    "output\\output_hw1\\data.json",
    "output\\output_hw1\\data.json",
];

pub fn homework2(rng: &mut ThreadRng) -> () {
    let hw2_start = Instant::now();
    println!("Homework 2 start!");

    let run_hw2question1 = true;
    let run_hw2question2 = true;
    let run_hw2matlab = true;

    if run_hw2question1 {
        hw2_question1(rng);
    }
    if run_hw2question2 {
        hw2_question2(rng);
    }
    if run_hw2matlab {
        publish_matlab_script(
            "scripts\\matlab_hw2.m",
            "matlab_hw2",
            "output\\output_hw2\\hw2_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw2_duration = hw2_start.elapsed();
    println!("Homework 2 finish! Time elapsed: {:?}", hw2_duration);
}

fn hw2_question1(_rng: &mut ThreadRng) -> () {}

fn hw2_question2(_rng: &mut ThreadRng) -> () {}

fn monte_carlo_sphere_a(d: usize, n_data: usize, rng: &mut ThreadRng) -> f64 {
    if d == 0 {
        return 1.0;
    }
    if d == 1 {
        return 2.0;
    }
    let x = (0..n_data)
        .map(|_| {
            let x_i = (0..(d / 2))
                .flat_map(|_| {
                    let r = (-2.0 * rng.random::<f64>().ln()).sqrt();
                    let t = TAU * rng.random::<f64>();
                    vec![r * t.cos(), r * t.sin()]
                })
                .take(d - 1)
                .collect::<Vec<f64>>();
            let sqrtx2 = x_i.iter().map(|x| x * x).sum::<f64>().sqrt();
            x_i.iter()
                .map(|x_i| (rng.random::<f64>().powf(1.0 / ((d - 1) as f64))) / sqrtx2 * x_i)
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();
    2.0 / (n_data as f64)
        * x.iter()
            .map(|x_i| (1.0 - x_i.iter().map(|&x_ik| x_ik * x_ik).sum::<f64>()).sqrt())
            .sum::<f64>()
        * monte_carlo_sphere_a(d - 1, n_data, rng)
}

fn monte_carlo_sphere_a2(d: usize, n_data: usize, rng: &mut ThreadRng) -> f64 {
    if d == 0 {
        return 1.0;
    }
    if d == 1 {
        return 2.0;
    }
    2.0 / (n_data as f64)
        * (0..n_data)
            .map(|_| (1.0 - rng.random::<f64>().powf(2.0 / ((d - 1) as f64))).sqrt())
            .sum::<f64>()
        * monte_carlo_sphere_a2(d - 1, n_data, rng)
}

fn monte_carlo_sphere_b(d: usize, n_data: usize, rng: &mut ThreadRng) -> f64 {
    if d == 0 {
        return 1.0;
    }
    if d == 1 {
        return 2.0;
    }
    let dm1 = d - 1;
    2.0 * 2.0f64.powi(dm1 as i32) / (n_data as f64)
        * (0..n_data)
            .map(|_| {
                let x2 = (0..dm1)
                    .map(|_| rng.random_range(-1.0..1.0))
                    .map(|x_k| x_k * x_k)
                    .sum::<f64>();
                if x2 <= 1.0 { (1.0 - x2).sqrt() } else { 0.0 }
            })
            .sum::<f64>()
}
