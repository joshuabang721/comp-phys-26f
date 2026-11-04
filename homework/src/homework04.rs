use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};
use std::time::Instant;

const PATHS: [&str; 5] = [
    "output\\output_hw4\\markov_chain.json",
    "output\\output_hw4\\heat_map_x1.json",
    "output\\output_hw4\\heat_map_x2.json",
    "output\\output_hw4\\lambda.json",
    "output\\output_hw4\\average.json",
];

pub fn homework4(rng: &mut ThreadRng) -> () {
    let hw4_start = Instant::now();
    println!("Homework 4 start!");

    let run_hw4question1 = true;
    let run_hw4question2 = true;
    let run_hw4matlab = true;

    if run_hw4question1 {
        hw4_question1(rng);
    }
    if run_hw4question2 {
        hw4_question2(rng);
    }
    if run_hw4matlab {
        publish_matlab_script(
            "scripts\\matlab_hw4.m",
            "matlab_hw4",
            "output\\output_hw4\\hw4_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw4_duration = hw4_start.elapsed();
    println!("Homework 4 finish! Time elapsed: {:?}", hw4_duration);
}

fn hw4_question1(rng: &mut ThreadRng) -> () {
    let hw4q1_start = Instant::now();
    println!("Hw 4 Question 1 start!");

    let c = f64::from_bits(4607430264483924253);
    let pdf = |x: f64| {
        let a = x * x - 1.0;
        c * (-4.0 * a * a).exp()
    };

    let x0 = 0.0;
    let step_length = 1.0;
    let n_init = 5000;
    let n_data = 50000;
    let chain = meteropolis_hastings_chain_1d(x0, step_length, &pdf, n_init, n_data, rng)
        .into_iter()
        .collect::<Vec<f64>>();

    write_data_to_file(&chain, PATHS[0], FileDataType::Json);

    let hw4q1_duration = hw4q1_start.elapsed();
    println!("Hw 4 Question 1 finish! Time elapsed: {:?}", hw4q1_duration);
}

fn hw4_question2(rng: &mut ThreadRng) -> () {
    let hw4q2_start = Instant::now();
    println!("Hw 4 Question 2 start!");

    let box_width = 1.0;
    let pdf = |(x1, x2): (f64, f64), l: f64| {
        if x1 < -box_width || x1 > box_width || x2 < -box_width || x2 > box_width {
            return 0.0;
        }
        (-l / (x1 - x2).abs()).exp()
    };

    let x0 = (0.0, 0.0);
    let step_length = 1.0;
    let lambda = 1.0;
    let n_init = 5000;
    let n_data = 50000;
    let (x1, x2) =
        meteropolis_hastings_chain_2d(x0, step_length, lambda, &pdf, n_init, n_data, rng)
            .into_iter()
            .unzip::<f64, f64, Vec<f64>, Vec<f64>>();

    write_data_to_file(&x1, PATHS[1], FileDataType::Json);
    write_data_to_file(&x2, PATHS[2], FileDataType::Json);

    let d_l = 0.05;
    let n_l = 40;
    let lambda = (0..=n_l).map(|i| d_l * (i as f64)).collect::<Vec<f64>>();
    let avg_dist = lambda
        .iter()
        .map(|l| {
            let x = meteropolis_hastings_chain_2d(
                (0.0, 0.0),
                step_length,
                *l,
                pdf,
                n_init,
                n_data,
                rng,
            );
            (1.0 / (x.len() as f64)) * x.iter().map(|(x1, x2)| (x1 - x2).abs()).sum::<f64>()
        })
        .collect::<Vec<f64>>();

    write_data_to_file(&lambda, PATHS[3], FileDataType::Json);
    write_data_to_file(&avg_dist, PATHS[4], FileDataType::Json);

    let hw4q2_duration = hw4q2_start.elapsed();
    println!("Hw 4 Question 2 finish! Time elapsed: {:?}", hw4q2_duration);
}

fn meteropolis_hastings_chain_1d<F>(
    x0: f64,
    step_length: f64,
    pdf: F,
    n_init: usize,
    n_data: usize,
    rng: &mut ThreadRng,
) -> Vec<f64>
where
    F: Fn(f64) -> f64,
{
    std::iter::once(x0)
        .chain(
            (0..(n_init + n_data))
                .scan((x0, pdf(x0)), |(x, pi_x), _| {
                    let l = rng.random_range(-step_length..step_length);
                    let y = *x + l;
                    let pi_y = pdf(y);
                    let p = (pi_y / *pi_x).min(1.0);
                    if rng.random::<f64>() < p {
                        *x = y;
                        *pi_x = pi_y;
                    }
                    Some(*x)
                })
                .skip(n_init),
        )
        .collect::<Vec<f64>>()
}

fn meteropolis_hastings_chain_2d<F>(
    x0: (f64, f64),
    step_length: f64,
    lambda: f64,
    pdf: F,
    n_init: usize,
    n_data: usize,
    rng: &mut ThreadRng,
) -> Vec<(f64, f64)>
where
    F: Fn((f64, f64), f64) -> f64,
{
    std::iter::once(x0)
        .chain(
            (0..(n_init + n_data))
                .scan((x0, pdf(x0, lambda)), |(x, pi_x), _| {
                    let l = (
                        rng.random_range(-step_length..step_length),
                        rng.random_range(-step_length..step_length),
                    );
                    let y = (x.0 + l.0, x.1 + l.1);
                    let pi_y = pdf(y, lambda);
                    let p = (pi_y / *pi_x).min(1.0);
                    if rng.random::<f64>() < p {
                        *x = y;
                        *pi_x = pi_y;
                    }
                    Some(*x)
                })
                .skip(n_init),
        )
        .collect::<Vec<(f64, f64)>>()
}
