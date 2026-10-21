use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};
use std::time::Instant;

pub fn homework1(rng: &mut ThreadRng) -> () {
    let hw1_start = Instant::now();
    println!("Homework 1 start!");

    hw1_question1(rng);
    hw1_question2();
    hw1_question3(rng);
    hw1_question4(rng);

    publish_matlab_script(
        "scripts\\matlab_hw1.m",
        "matlab_hw1",
        "output\\output_hw1\\hw1_document.pdf",
        vec![
            "output\\output_hw1\\prng_samples.json",
            "output\\output_hw1\\lcg_samples.json",
            "output\\output_hw1\\prng_vector_samples.json",
            "output\\output_hw1\\lcg_vector_samples.json",
            "output\\output_hw1\\prng_autocorrelation.json",
            "output\\output_hw1\\lcg_autocorrelation.json",
        ],
    );

    let hw1_duration = hw1_start.elapsed();
    println!("Homework 1 finish! Time elapsed: {:?}", hw1_duration);
}

fn hw1_question1(rng: &mut ThreadRng) -> () {
    let hw1q1_start = Instant::now();
    println!("Hw 1 question 1 start!");

    let n = 10000;
    let prng_samples = rng.random_iter::<f64>().take(n).collect::<Vec<f64>>();

    write_data_to_file(
        &prng_samples,
        "output\\output_hw1\\prng_samples.json",
        FileDataType::Json,
    );

    let hw1q1_duration = hw1q1_start.elapsed();
    println!("Hw 1 question 1 finish! Time elapsed: {:?}", hw1q1_duration);
}

fn hw1_question2() -> () {
    let hw1q2_start = Instant::now();
    println!("Hw 1 question 2 start!");

    let n = 10000;
    let a = 63;
    let c = 319;
    let m = 65537;

    let seed = 26827;
    let lcg = linear_congruential_generator(a, c, m);

    let mut x_i = seed;
    let lcg_samples: Vec<f64> = (0..n)
        .map(|_| {
            x_i = lcg(x_i);
            (x_i as f64) / (m as f64)
        })
        .collect();

    write_data_to_file(
        &lcg_samples,
        "output\\output_hw1\\lcg_samples.json",
        FileDataType::Json,
    );

    let hw1q2_duration = hw1q2_start.elapsed();
    println!("Hw 1 question 2 finish! Time elapsed: {:?}", hw1q2_duration);
}

fn hw1_question3(rng: &mut ThreadRng) -> () {
    let hw1q3_start = Instant::now();
    println!("Hw 1 question 3 start!");

    let n = 100000;

    let prng_vector_samples = (0..3)
        .map(|_| rng.random_iter::<f64>().take(n).collect::<Vec<f64>>())
        .collect::<Vec<Vec<f64>>>();

    let a = 63;
    let c = 319;
    let m = 65537;

    let seed = [4022, 20152, 57038];
    let lcg = linear_congruential_generator(a, c, m);

    let lcg_vector_samples = seed
        .iter()
        .map(|&x0| {
            let mut x_i = x0;
            (0..n)
                .map(|_| {
                    x_i = lcg(x_i);
                    (x_i as f64) / (m as f64)
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    write_data_to_file(
        &prng_vector_samples,
        "output\\output_hw1\\prng_vector_samples.json",
        FileDataType::Json,
    );
    write_data_to_file(
        &lcg_vector_samples,
        "output\\output_hw1\\lcg_vector_samples.json",
        FileDataType::Json,
    );

    let hw1q3_duration = hw1q3_start.elapsed();
    println!("Hw 1 question 3 finish! Time elapsed: {:?}", hw1q3_duration);
}

fn hw1_question4(rng: &mut ThreadRng) -> () {
    let hw1q4_start = Instant::now();
    println!("Hw 1 question 4 start!");

    let len = 10000;
    let n_max = 5000;

    let prng_samples = rng.random_iter::<f64>().take(len).collect::<Vec<f64>>();

    let a = 63;
    let c = 319;
    let m = 65537;

    let seed = 44983;
    let lcg = linear_congruential_generator(a, c, m);

    let mut x_i = seed;
    let lcg_samples: Vec<f64> = (0..len)
        .map(|_| {
            x_i = lcg(x_i);
            (x_i as f64) / (m as f64)
        })
        .collect();

    let prng_autocorrelation_function = autocorrelation_function(&prng_samples);
    let lcg_autocorrelation_function = autocorrelation_function(&lcg_samples);

    let prng_autocorrelation = (1..n_max)
        .map(|n| prng_autocorrelation_function(n))
        .collect::<Vec<f64>>();
    let lcg_autocorrelation = (1..n_max)
        .map(|n| lcg_autocorrelation_function(n))
        .collect::<Vec<f64>>();

    write_data_to_file(
        &prng_autocorrelation,
        "output\\output_hw1\\prng_autocorrelation.json",
        FileDataType::Json,
    );
    write_data_to_file(
        &lcg_autocorrelation,
        "output\\output_hw1\\lcg_autocorrelation.json",
        FileDataType::Json,
    );

    let hw1q4_duration = hw1q4_start.elapsed();
    println!("Hw 1 question 4 finish! Time elapsed: {:?}", hw1q4_duration);
}

fn linear_congruential_generator(a: u64, c: u64, m: u64) -> impl Fn(u64) -> u64 {
    move |x: u64| (a * x + c) % m
}

fn autocorrelation_function(x: &Vec<f64>) -> impl Fn(usize) -> f64 {
    let len = x.len();
    let mean = (1.0 / (len as f64)) * x.iter().sum::<f64>();
    move |n: usize| {
        if n >= len {
            0.0
        } else {
            let xkxkn = (0..(len - n)).map(|k| x[k] * x[k + n]).sum::<f64>();
            (1.0 / ((len - n) as f64)) * xkxkn - mean * mean
        }
    }
}
