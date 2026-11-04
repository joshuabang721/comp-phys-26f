use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::{RngExt, rngs::ThreadRng};
use std::{array, time::Instant};

const PATHS: [&str; 9] = [
    "output\\output_hw5\\transition_graph.json",
    "output\\output_hw5\\transition_matrix_1b.json",
    "output\\output_hw5\\temperature1.json",
    "output\\output_hw5\\transition_matrix_1c.json",
    "output\\output_hw5\\temperature2.json",
    "output\\output_hw5\\energy_density_t.json",
    "output\\output_hw5\\heat_capacity_t.json",
    "output\\output_hw5\\magnet_density_t.json",
    "output\\output_hw5\\magnet_sus_t.json",
];

const J: f64 = 1.0;
const H: f64 = 0.0;
const KB: f64 = 1.0;

pub fn homework5(rng: &mut ThreadRng) -> () {
    let hw5_start = Instant::now();
    println!("Homework 5 start!");

    let run_hw5question1 = false;
    let run_hw5question2 = false;
    let run_hw5question3 = false;
    let run_hw5matlab = true;

    if run_hw5question1 {
        hw5_question1();
    }
    if run_hw5question2 {
        hw5_question2(rng);
    }
    if run_hw5question3 {
        hw5_question3();
    }
    if run_hw5matlab {
        publish_matlab_script(
            "scripts\\matlab_hw5.m",
            "matlab_hw5",
            "output\\output_hw5\\hw5_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw5_duration = hw5_start.elapsed();
    println!("Homework 5 finish! Time elapsed: {:?}", hw5_duration);
}

fn hw5_question1() -> () {
    let hw5q1_start = Instant::now();
    println!("Hw 5 question 1 start!");

    let beta = 1.0;
    let transition_graph = transition_graph_3(beta).iter().fold(
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

    let beta1 = [10.0, 1.0, 0.1];
    let transition_matrices_b = beta1.map(|b| transition_matrix_3(b));

    write_data_to_file(&transition_matrices_b, PATHS[1], FileDataType::Json);

    let dtemp = 0.01;
    let t0 = 0.75;
    let temperature = (0..=125)
        .map(|i| t0 + (i as f64) * dtemp)
        .collect::<Vec<f64>>();
    let beta2 = temperature
        .iter()
        .map(|&t| 1.0 / (KB * t))
        .collect::<Vec<f64>>();

    let transition_matrix_c = beta2
        .iter()
        .map(|&b| transition_matrix_3(b))
        .collect::<Vec<[[f64; 8]; 8]>>();

    write_data_to_file(&temperature, PATHS[2], FileDataType::Json);
    write_data_to_file(&transition_matrix_c, PATHS[3], FileDataType::Json);

    let hw5q1_duration = hw5q1_start.elapsed();
    println!("Hw 5 question 1 finish! Time elapsed: {:?}", hw5q1_duration);
}

fn hw5_question2(rng: &mut ThreadRng) -> () {
    let hw5q2_start = Instant::now();
    println!("Hw 5 question 2 start!");

    let n_spin = 50;

    let energy = |x: &Vec<i32>| {
        let len = x.len();
        -J * ((0..len)
            .map(|i| {
                if i == 0 {
                    x[0] * x[len - 1]
                } else {
                    x[i] * x[i - 1]
                }
            })
            .sum::<i32>() as f64)
            + -H * (x.iter().sum::<i32>() as f64)
    };
    let magnet = |x: &Vec<i32>| x.iter().sum::<i32>() as f64;

    let n_data = 50000;
    let n_init = 5000;

    let n_temp = 250;
    let d_temp = 0.02;
    let temperature = (0..n_temp)
        .map(|i| d_temp * ((i + 1) as f64))
        .collect::<Vec<f64>>();

    let (e_tn, m_tn) = temperature
        .iter()
        .map(|t| markov_chain(n_data, n_init, n_spin, 1.0 / (KB * t), energy, magnet, rng))
        .unzip::<Vec<f64>, Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>>();

    let avg_e_t = e_tn
        .iter()
        .map(|e_n| e_n.iter().sum::<f64>() / (n_data as f64))
        .collect::<Vec<f64>>();
    let avg_m_t = m_tn
        .iter()
        .map(|m_n| m_n.iter().sum::<f64>() / (n_data as f64))
        .collect::<Vec<f64>>();
    let avg_e2_t = e_tn
        .iter()
        .map(|e_n: &Vec<f64>| e_n.iter().map(|&e| e * e).sum::<f64>() / (n_data as f64))
        .collect::<Vec<f64>>();
    let avg_m2_t = m_tn
        .iter()
        .map(|m_n| m_n.iter().map(|&m| m * m).sum::<f64>() / (n_data as f64))
        .collect::<Vec<f64>>();

    let energy_density_t = avg_e_t
        .iter()
        .map(|&avg_e| avg_e / (n_spin as f64))
        .collect::<Vec<f64>>();
    let heat_capacity_t = (0..n_temp)
        .map(|t| {
            (avg_e2_t[t] - avg_e_t[t] * avg_e_t[t])
                / ((n_spin as f64) * KB * temperature[t] * temperature[t])
        })
        .collect::<Vec<f64>>();
    let magnet_density_t = avg_m_t
        .iter()
        .map(|&avg_m| avg_m / (n_spin as f64))
        .collect::<Vec<f64>>();
    let magnet_sus_t = (0..n_temp)
        .map(|t| (avg_m2_t[t] - avg_m_t[t] * avg_m_t[t]) / ((n_spin as f64) * KB * temperature[t]))
        .collect::<Vec<f64>>();

    write_data_to_file(&temperature, PATHS[4], FileDataType::Json);
    write_data_to_file(&energy_density_t, PATHS[5], FileDataType::Json);
    write_data_to_file(&heat_capacity_t, PATHS[6], FileDataType::Json);
    write_data_to_file(&magnet_density_t, PATHS[7], FileDataType::Json);
    write_data_to_file(&magnet_sus_t, PATHS[8], FileDataType::Json);

    let hw5q2_duration = hw5q2_start.elapsed();
    println!("Hw 5 question 2 finish! Time elapsed: {:?}", hw5q2_duration);
}

fn hw5_question3() -> () {
    let hw5q3_start = Instant::now();
    println!("Hw 5 question 3 start!");

    let hw5q3_duration = hw5q3_start.elapsed();
    println!("Hw 5 question 3 finish! Time elapsed: {:?}", hw5q3_duration);
}

fn transition_graph_3(beta: f64) -> Vec<(usize, usize, f64)> {
    (0..8)
        .flat_map(|x_idx| {
            let x: [i32; 3] = array::from_fn(|i| if ((x_idx >> i) & 1) == 1 { 1 } else { -1 });
            let mut p_inv = 1.0;
            (0..=3).map(move |s| {
                if s < 3 {
                    let y_idx = x_idx ^ (1 << s);
                    let y: [i32; 3] =
                        array::from_fn(|i| if ((y_idx >> i) & 1) == 1 { 1 } else { -1 });
                    let p = transition_probability_3(x, y, beta) / 3.0;
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

fn transition_matrix_3(beta: f64) -> [[f64; 8]; 8] {
    transition_graph_3(beta)
        .iter()
        .fold([[0.0; 8]; 8], |mut acc, (x, y, p)| {
            acc[*y][*x] += p;
            acc
        })
}

fn transition_probability_3(x: [i32; 3], y: [i32; 3], beta: f64) -> f64 {
    let e = configuration_energy_3;
    (-beta * ((e(y) - e(x)) as f64)).exp().min(1.0)
}

fn configuration_energy_3(x: [i32; 3]) -> i32 {
    -(x[0] * x[1] + x[1] * x[2] + x[2] * x[0])
}

fn markov_chain<E, M>(
    n_data: usize,
    n_init: usize,
    n_spin: usize,
    beta: f64,
    energy: E,
    magnet: M,
    rng: &mut ThreadRng,
) -> (Vec<f64>, Vec<f64>)
where
    E: Fn(&Vec<i32>) -> f64,
    M: Fn(&Vec<i32>) -> f64,
{
    let mut x_n = Vec::new();
    let mut e_n = 0.0;
    let mut m_n = 0.0;
    (0..=(n_data + n_init))
        .map(|i| {
            if i == 0 {
                x_n = vec![if rng.random_bool(0.5) { 1 } else { -1 }; n_spin];
                e_n = energy(&x_n);
                m_n = magnet(&x_n);
            } else {
                let idx = rng.random_range(0..n_spin);
                let y = x_n
                    .iter()
                    .enumerate()
                    .map(|(i, &e)| if i == idx { -e } else { e })
                    .collect::<Vec<i32>>();
                let e_y = energy(&y);
                let p = (-beta * (e_y - e_n)).exp().min(1.0);
                if rng.random::<f64>() < p {
                    x_n = y;
                    e_n = e_y;
                    m_n = magnet(&x_n);
                }
            };
            (e_n, m_n)
        })
        .skip(n_init)
        .unzip()
}
