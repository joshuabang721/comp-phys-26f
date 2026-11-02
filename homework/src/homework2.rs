use dataio::dataio::{publish_matlab_script, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};
use std::{
    f64::consts::{PI, TAU},
    time::Instant,
};

const PATHS: [&str; 7] = [
    "output\\output_hw2\\volume_sim.json",
    "output\\output_hw2\\volume_true.json",
    "output\\output_hw2\\particle_1.json",
    "output\\output_hw2\\particle_2.json",
    "output\\output_hw2\\accepted.json",
    "output\\output_hw2\\lambda.json",
    "output\\output_hw2\\average_distance.json",
];

const V_D_TRUE: [f64; 9] = [
    1.0,
    2.0,
    PI,
    4.0 * PI / 3.0,
    PI * PI / 2.0,
    8.0 * PI * PI / 15.0,
    PI * PI * PI / 6.0,
    16.0 * PI * PI * PI / 105.0,
    PI * PI * PI * PI / 24.0,
];

pub fn homework2(rng: &mut ThreadRng) -> () {
    let hw2_start = Instant::now();
    println!("Homework 2 start!");

    let run_hw2question1 = true;
    let run_hw2question2 = true;
    let run_hw2matlab = false;

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

fn hw2_question1(rng: &mut ThreadRng) -> () {
    let hw2q1_start = Instant::now();
    println!("Hw 2 Question 1 start!");

    let n_data = 10000;

    let v_d_sim = (0..9)
        .scan(0.0, |state, dim| {
            *state = match dim {
                0 => 1.0,
                1 => 2.0,
                d => {
                    2.0 / (n_data as f64)
                        * (0..n_data)
                            .map(|_| {
                                (1.0 - rng.random::<f64>().powf(2.0 / ((d - 1) as f64))).sqrt()
                            })
                            .sum::<f64>()
                        * *state
                }
            };
            Some(*state)
        })
        .collect::<Vec<f64>>();
    let v_d_true = V_D_TRUE.to_vec();

    write_data_to_file(&v_d_sim, PATHS[0], dataio::dataio::FileDataType::Json);
    write_data_to_file(&v_d_true, PATHS[1], dataio::dataio::FileDataType::Json);

    let hw2q1_duration = hw2q1_start.elapsed();
    println!("Hw 2 Question 1 finish! Time elapsed: {:?}", hw2q1_duration);
}

fn hw2_question2(rng: &mut ThreadRng) -> () {
    let hw2q2_start = Instant::now();
    println!("Hw 2 Question 2 start!");

    let n_data = 10000;
    let pdf = |(x1, x2): (f64, f64), l: f64| {
        if x1 < -1.0 || x1 > 1.0 || x2 < -1.0 || x2 > 1.0 {
            0.0
        } else {
            (-l / (x1 - x2).abs()).exp()
        }
    };

    let lambda = 1.0;
    let (x1, x2, r) = rejection_sampling(n_data, lambda, pdf, rng);

    write_data_to_file(&x1, PATHS[2], dataio::dataio::FileDataType::Json);
    write_data_to_file(&x2, PATHS[3], dataio::dataio::FileDataType::Json);
    write_data_to_file(&r, PATHS[4], dataio::dataio::FileDataType::Json);

    let n_data = 100000;
    let d_l = 0.05;
    let n_l = 40;
    let lambda = (0..=n_l).map(|i| d_l * (i as f64)).collect::<Vec<f64>>();
    let x_avg = lambda
        .iter()
        .map(|&l| {
            let (x1, x2, _) = rejection_sampling(n_data, l, pdf, rng);
            std::iter::zip(x1, x2)
                .map(|(x1, x2)| (x1 - x2).abs())
                .sum::<f64>()
                / (n_data as f64)
        })
        .collect::<Vec<f64>>();

    write_data_to_file(&lambda, PATHS[5], dataio::dataio::FileDataType::Json);
    write_data_to_file(&x_avg, PATHS[6], dataio::dataio::FileDataType::Json);

    let hw2q2_duration = hw2q2_start.elapsed();
    println!("Hw 2 Question 2 finish! Time elapsed: {:?}", hw2q2_duration);
}

fn _monte_carlo_sphere_a(d: usize, n_data: usize, rng: &mut ThreadRng) -> f64 {
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
        * _monte_carlo_sphere_a(d - 1, n_data, rng)
}

fn _monte_carlo_sphere_a2(d: usize, n_data: usize, rng: &mut ThreadRng) -> f64 {
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
        * _monte_carlo_sphere_a2(d - 1, n_data, rng)
}

fn _monte_carlo_sphere_b(d: usize, n_data: usize, rng: &mut ThreadRng) -> f64 {
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

fn rejection_sampling<F>(
    n_data: usize,
    l: f64,
    pdf: F,
    rng: &mut ThreadRng,
) -> (Vec<f64>, Vec<f64>, f64)
where
    F: Fn((f64, f64), f64) -> f64,
{
    let m = (-l / 2.0).exp();
    let mut n_acc = 0;
    let (x1, x2) = (0..n_data)
        .map(|_| {
            let x1 = rng.random_range(-1.0..1.0);
            let x2 = rng.random_range(-1.0..1.0);
            if rng.random_range(0.0..m) < pdf((x1, x2), l) {
                n_acc += 1;
                Some((x1, x2))
            } else {
                None
            }
        })
        .flatten()
        .unzip::<f64, f64, Vec<f64>, Vec<f64>>();
    (x1, x2, ((n_acc as f64) / (n_data as f64)))
}
