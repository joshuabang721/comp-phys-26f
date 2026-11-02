use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::{f64::consts::PI, time::Instant};

const PATHS: [&str; 2] = [
    "output\\output_hw9\\time.json",
    "output\\output_hw9\\recurrence.json",
];

pub fn homework9(_rng: &mut ThreadRng) -> () {
    let hw9_start = Instant::now();
    println!("Homework 9 start!");

    let run_hw9question1 = true;
    let run_hw9question2 = true;
    let run_hw9matlab = true;

    if run_hw9question1 {
        hw9_question1();
    }
    if run_hw9question2 {
        hw9_question2();
    }
    if run_hw9matlab {
        publish_matlab_script(
            "scripts\\matlab_hw9.m",
            "matlab_hw9",
            "output\\output_hw9\\hw9_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw9_duration = hw9_start.elapsed();
    println!("Homework 9 finish! Time elapsed: {:?}", hw9_duration);
}

fn hw9_question1() -> () {
    let hw9q1_start = Instant::now();
    println!("Hw 9 question 1 start!");

    let n = 32;
    let m = 1.0;
    let k = 1.0;
    let alpha = 1.0;
    let dt = 0.025;
    let t_min = 0.0f64;
    let t_max = 1200.0f64;
    let t_num = ((t_max - t_min) / dt).round().abs() as usize;
    let t = (0..t_num)
        .map(|i| t_min + (i as f64) * dt)
        .collect::<Vec<f64>>();
    write_data_to_file(&t, PATHS[0], FileDataType::Json);

    let a = |x_i: &Vec<f64>| {
        let i_max = x_i.len() - 1;
        x_i.iter()
            .enumerate()
            .map(|(i, _)| {
                let (im1, ip1) = match i {
                    0 => (i_max, 1),
                    i if i == i_max => (i - 1, 0),
                    i => (i - 1, i + 1),
                };
                let (xim1, xi, xip1) = (x_i[im1], x_i[i], x_i[ip1]);
                let (a, b) = (xip1 - xi, xi - xim1);
                1.0 / m * (k * (xip1 + xim1 - 2.0 * xi) + alpha * (a * a - b * b))
            })
            .collect::<Vec<f64>>()
    };

    let x_0_i = (0..n)
        .map(|i| (2.0 * PI * (i as f64) / (n as f64)).cos())
        .collect::<Vec<f64>>();
    let v_0_i = (0..n).map(|_| 0.0).collect::<Vec<f64>>();
    let a_0_i = a(&x_0_i);

    let (x_t_i, v_t_i) = t
        .iter()
        .scan((x_0_i, v_0_i, a_0_i), |(x_i, v_i, a_i), _| {
            *x_i = x_i
                .iter()
                .enumerate()
                .map(|(i, x)| x + v_i[i] * dt + 0.5 * a_i[i] * dt * dt)
                .collect::<Vec<f64>>();
            let anp1_i = a(x_i);
            *v_i = v_i
                .iter()
                .enumerate()
                .map(|(i, v)| v + 0.5 * (a_i[i] + anp1_i[i]) * dt)
                .collect::<Vec<f64>>();
            *a_i = anp1_i;
            Some((x_i.clone(), v_i.clone()))
        })
        .unzip::<Vec<f64>, Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>>();

    write_data_to_file(
        &(x_t_i[t_num - 1]),
        "output\\output_hw9\\position.json",
        FileDataType::Json,
    );
    write_data_to_file(
        &(v_t_i[t_num - 1]),
        "output\\output_hw9\\velocity.json",
        FileDataType::Json,
    );

    let omega1 = 2.0 * (k / m).sqrt() * (PI / (n as f64)).sin();
    let epsilon_c =
        |i: usize| (2.0 / (n as f64)).sqrt() * (2.0 * PI * (i as f64) / (n as f64)).cos();
    let epsilon_s =
        |i: usize| (2.0 / (n as f64)).sqrt() * (2.0 * PI * (i as f64) / (n as f64)).sin();

    let qc_n = x_t_i
        .iter()
        .map(|x_i| x_i.iter().enumerate().map(|(i, &x)| x * epsilon_c(i)).sum())
        .collect::<Vec<f64>>();
    let qs_n = x_t_i
        .iter()
        .map(|x_i| x_i.iter().enumerate().map(|(i, &x)| x * epsilon_s(i)).sum())
        .collect::<Vec<f64>>();
    let qdc_n = v_t_i
        .iter()
        .map(|v_i| v_i.iter().enumerate().map(|(i, &v)| v * epsilon_c(i)).sum())
        .collect::<Vec<f64>>();
    let qds_n = v_t_i
        .iter()
        .map(|v_i| v_i.iter().enumerate().map(|(i, &v)| v * epsilon_s(i)).sum())
        .collect::<Vec<f64>>();

    let e1_t = (0..t_num)
        .map(|n| {
            let (qc, qs, qdc, qds) = (qc_n[n], qs_n[n], qdc_n[n], qds_n[n]);
            m / 2.0 * (omega1 * omega1 * (qc * qc + qs * qs) + qdc * qdc + qds * qds)
        })
        .collect::<Vec<f64>>();
    let e1_0 = e1_t[0];

    let recurrence = e1_t.iter().map(|e1| e1 / e1_0).collect::<Vec<f64>>();

    write_data_to_file(&recurrence, PATHS[1], FileDataType::Json);

    let hw9q1_duration = hw9q1_start.elapsed();
    println!("Hw 9 question 1 finish! Time elapsed: {:?}", hw9q1_duration);
}

fn hw9_question2() -> () {
    let hw9q2_start = Instant::now();
    println!("Hw 9 question 2 start!");

    // write_data_to_file(&error, PATHS[4], FileDataType::Json);

    let hw9q2_duration = hw9q2_start.elapsed();
    println!("Hw 9 question 2 finish! Time elapsed: {:?}", hw9q2_duration);
}
