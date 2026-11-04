use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::time::Instant;

const PATHS: [&str; 2] = [
    "output\\output_hw10\\u_x_t.json",
    "output\\output_hw10\\u_0_t.json",
];

pub fn homework10(_rng: &mut ThreadRng) -> () {
    let hw10_start = Instant::now();
    println!("Homework 10 start!");

    let run_hw10question1 = true;
    let run_hw10question2 = false;
    let run_hw10matlab = false;

    if run_hw10question1 {
        hw10_question1();
    }
    if run_hw10question2 {
        hw10_question2();
    }
    if run_hw10matlab {
        publish_matlab_script(
            "scripts\\matlab_hw10.m",
            "matlab_hw10",
            "output\\output_hw10\\hw10_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw10_duration = hw10_start.elapsed();
    println!("Homework 10 finish! Time elapsed: {:?}", hw10_duration);
}

fn hw10_question1() -> () {
    let hw10q1_start = Instant::now();
    println!("Hw 10 question 1 start!");

    let d = 0.5;
    let dx = 0.1;
    let dt = 1.0;
    let r = (d * dt) / (dx * dx);

    let (t_min, t_max) = (0.0, 60.0);
    let t_num = ((t_max - t_min) / dt) as usize;
    let t_n = (0..=t_num)
        .map(|n| t_min + (n as f64) * dt)
        .collect::<Vec<f64>>();
    let times = [0.0, 15.0, 30.0, 45.0, 60.0];

    let (x_min, x_max) = (-60.0, 60.0);
    let x_num = ((x_max - x_min) / dx) as usize;
    let x_j = (0..=x_num)
        .map(|n| x_min + (n as f64) * dx)
        .collect::<Vec<f64>>();

    let w = 10.0;
    let u_x_0 = |x: f64| (-x * x / (w * w)).exp();

    let mut u_0_t = Vec::<f64>::with_capacity(t_num + 1);

    let a = -r / 2.0;
    let b = 1.0 + r;
    let c = -r / 2.0;

    let u_x_t = t_n
        .iter()
        .enumerate()
        .scan(Vec::new(), |u_j, (n, t)| {
            *u_j = if n == 0 {
                x_j.iter().map(|&x| u_x_0(x)).collect::<Vec<f64>>()
            } else {
                let d_j = u_j
                    .iter()
                    .enumerate()
                    .map(|(j, &uj)| {
                        let j_max = u_j.len() - 1;
                        let (ujm1, ujp1) = match j {
                            0 => (0.0, u_j[1]),
                            j if j == j_max => (u_j[j_max - 1], 0.0),
                            j => (u_j[j - 1], u_j[j + 1]),
                        };
                        r / 2.0 * ujm1 + (1.0 - r) * uj + r / 2.0 * ujp1
                    })
                    .collect::<Vec<f64>>();
                thomas_algorithm(a, b, c, &d_j)
            };
            u_0_t.push(u_j[0]);
            Some(if times.contains(t) {
                Some((*u_j).clone())
            } else {
                None
            })
        })
        .flatten()
        .collect::<Vec<Vec<f64>>>();

    write_data_to_file(&u_x_t, PATHS[0], FileDataType::Json);
    write_data_to_file(&u_0_t, PATHS[1], FileDataType::Json);

    let hw10q1_duration = hw10q1_start.elapsed();
    println!(
        "Hw 10 question 1 finish! Time elapsed: {:?}",
        hw10q1_duration
    );
}

fn hw10_question2() -> () {
    let hw10q2_start = Instant::now();
    println!("Hw 10 question 2 start!");

    // write_data_to_file(&error, PATHS[4], FileDataType::Json);

    let hw10q2_duration = hw10q2_start.elapsed();
    println!(
        "Hw 10 question 2 finish! Time elapsed: {:?}",
        hw10q2_duration
    );
}

fn thomas_algorithm(
    lower_diagonal_a: f64,
    main_diagonal_b: f64,
    upper_diagonal_c: f64,
    rhs_vector_d_j: &[f64],
) -> Vec<f64> {
    let a = lower_diagonal_a;
    let b = main_diagonal_b;
    let c = upper_diagonal_c;
    let d_j = rhs_vector_d_j;
    let j_max = d_j.len() - 1;

    d_j.iter()
        .enumerate()
        .scan((0.0, 0.0), |(cjm1, djm1), (j, dj)| {
            (*cjm1, *djm1) = match j {
                0 => (0.0, 0.0),
                1 => (c / b, dj / b),
                j if j == j_max => (0.0, 0.0),
                _ => {
                    let q = b - a * *cjm1;
                    (c / q, (dj - a * *djm1) / q)
                }
            };
            Some((*cjm1, *djm1))
        })
        .collect::<Vec<(f64, f64)>>()
        .iter()
        .rev()
        .enumerate()
        .scan(0.0, |ujp1, (j, (chat, dhat))| {
            *ujp1 = match j {
                0 => 0.0,
                1 => *dhat,
                j if j == j_max => 0.0,
                _ => *dhat - *chat * *ujp1,
            };
            Some(*ujp1)
        })
        .collect::<Vec<f64>>()
        .into_iter()
        .rev()
        .collect::<Vec<f64>>()
}
