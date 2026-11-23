use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::{f64::consts::PI, time::Instant};

const PATHS: [&str; 8] = [
    "output\\output_hw07\\p1_time.json",
    "output\\output_hw07\\p1_x0.json",
    "output\\output_hw07\\p1_x0true.json",
    "output\\output_hw07\\p1_ek.json",
    "output\\output_hw07\\p1_ep.json",
    "output\\output_hw07\\p1_et.json",
    "output\\output_hw07\\p1_dt.json",
    "output\\output_hw07\\p1_epsilon.json",
];

pub fn homework07v2(_rng: &mut ThreadRng) -> () {
    let hw07_start = Instant::now();
    println!("Homework 07v2 start!");

    let run_hw07problem1 = true;
    let run_hw07problem2 = false;
    let run_hw07problem3 = false;
    let run_hw07matlab = true;

    if run_hw07problem1 {
        hw07_problem1();
    }
    if run_hw07problem2 {
        hw07_problem2();
    }
    if run_hw07problem3 {
        hw07_problem3();
    }
    if run_hw07matlab {
        publish_matlab_script(
            "scripts\\matlab_hw07v2.m",
            "matlab_hw07v2",
            "output\\output_hw07\\hw07v2_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw07_duration = hw07_start.elapsed();
    println!("Homework 07v2 finish! Time elapsed: {:?}", hw07_duration);
}

fn hw07_problem1() -> () {
    let hw07p1_start = Instant::now();
    println!("Hw 07v2 problem 1 start!");

    let alpha = 0.0;
    let n = 32;
    let m: f64 = 1.0;
    let k: f64 = 1.0;

    let q_l = |l: usize| 2.0 * PI / (n as f64) * (l as f64);
    let omega_l = |l: usize| 2.0 * (k / m).sqrt() * ((q_l(l) / 2.0).sin()).abs();

    let amp = 1.0;

    let (x0_i, v0_i) = (0..n)
        .map(|i| (amp * (2.0 * PI / (n as f64) * (i as f64)).cos(), 0.0))
        .unzip::<f64, f64, Vec<f64>, Vec<f64>>();
    let a_i = |x_i: &[f64], _v_i: &[f64]| {
        let imax = x_i.len() - 1;
        x_i.iter()
            .enumerate()
            .map(|(i, &xi)| {
                let (im1, ip1) = match i {
                    0 => (imax, 1),
                    i if i == imax => (imax - 1, 0),
                    i => (i - 1, i + 1),
                };
                let (xim1, xip1) = (x_i[im1], x_i[ip1]);
                (k * (xip1 + xim1 - 2.0 * xi)
                    + alpha
                        * ({
                            let (a, b) = (xip1 - xi, xi - xim1);
                            a * a - b * b
                        }))
                    / m
            })
            .collect::<Vec<f64>>()
    };

    let dt = [0.2, 0.1, 0.05, 0.025];
    let t_max = 100;

    let epsilonmax_dt = dt
        .iter()
        .map(|&dt| {
            let nt = ((t_max as f64) / dt).round() as usize;
            let t = (0..=nt).map(|t| (t as f64) * dt).collect::<Vec<f64>>();
            let (x0ek_t, epet_t) = t
                .iter()
                .enumerate()
                .scan(
                    (x0_i.clone(), v0_i.clone(), a_i(&x0_i, &v0_i)),
                    |(xt_i, vt_i, at_i), (n, _)| {
                        if n != 0 {
                            for (i, &ati) in at_i.iter().enumerate() {
                                xt_i[i] = xt_i[i] + vt_i[i] * dt + 0.5 * ati * dt * dt;
                            }
                            let at2_i = a_i(xt_i, vt_i);
                            for (i, (&at1i, &at2i)) in at_i.iter().zip(at2_i.iter()).enumerate() {
                                vt_i[i] = vt_i[i] + 0.5 * (at1i + at2i) * dt;
                            }
                            *at_i = at2_i;
                        }
                        let xt0 = xt_i[0];
                        let ek = m / 2.0 * vt_i.iter().map(|&vti| vti * vti).sum::<f64>();
                        let ep = k / 2.0
                            * xt_i
                                .iter()
                                .enumerate()
                                .map(|(i, xti)| {
                                    let im1 = if i == 0 { xt_i.len() - 1 } else { i - 1 };
                                    let a = xti - xt_i[im1];
                                    a * a
                                })
                                .sum::<f64>();
                        let et = ek + ep;
                        Some(((xt0, ek), (ep, et)))
                    },
                )
                .unzip::<(f64, f64), (f64, f64), Vec<(f64, f64)>, Vec<(f64, f64)>>();
            let (ep_t, et_t) = epet_t.into_iter().unzip::<f64, f64, Vec<f64>, Vec<f64>>();
            if dt == 0.05 {
                write_data_to_file(&t, PATHS[0], FileDataType::Json);
                let (x0_t, ek_t) = x0ek_t.into_iter().unzip::<f64, f64, Vec<f64>, Vec<f64>>();
                write_data_to_file(&x0_t, PATHS[1], FileDataType::Json);
                let x0true_t = t
                    .iter()
                    .map(|&t| amp * (omega_l(1) * t).cos())
                    .collect::<Vec<f64>>();
                write_data_to_file(&x0true_t, PATHS[2], FileDataType::Json);
                write_data_to_file(&ek_t, PATHS[3], FileDataType::Json);
                write_data_to_file(&ep_t, PATHS[4], FileDataType::Json);
                write_data_to_file(&et_t, PATHS[5], FileDataType::Json);
                let _ = ep_t;
            }
            let et_0 = et_t[0];
            et_t.iter().fold(f64::MIN, |epsilonmax, &ett| {
                epsilonmax.max((ett - et_0).abs() / et_0)
            })
        })
        .collect::<Vec<f64>>();

    write_data_to_file(&dt, PATHS[6], FileDataType::Json);
    write_data_to_file(&epsilonmax_dt, PATHS[7], FileDataType::Json);

    let hw07p1_duration = hw07p1_start.elapsed();
    println!(
        "Hw 07v2 problem 1 finish! Time elapsed: {:?}",
        hw07p1_duration
    );
}

fn hw07_problem2() -> () {
    let hw07p2_start = Instant::now();
    println!("Hw 07v2 problem 2 start!");

    let hw07p2_duration = hw07p2_start.elapsed();
    println!(
        "Hw 07v2 problem 2 finish! Time elapsed: {:?}",
        hw07p2_duration
    );
}
fn hw07_problem3() -> () {
    let hw07p3_start = Instant::now();
    println!("Hw 07v2 problem 3 start!");

    let hw07p3_duration = hw07p3_start.elapsed();
    println!(
        "Hw 07v2 problem 3 finish! Time elapsed: {:?}",
        hw07p3_duration
    );
}
