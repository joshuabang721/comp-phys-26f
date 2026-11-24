use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::{f64::consts::PI, time::Instant};

const PATHS: [&str; 23] = [
    "output\\output_hw07\\p1_time.json",
    "output\\output_hw07\\p1_x0.json",
    "output\\output_hw07\\p1_x0true.json",
    "output\\output_hw07\\p1_ek.json",
    "output\\output_hw07\\p1_ep.json",
    "output\\output_hw07\\p1_et.json",
    "output\\output_hw07\\p1_dt.json",
    "output\\output_hw07\\p1_epsilon.json",
    "output\\output_hw07\\p2_time.json",
    "output\\output_hw07\\p2_epst00.json",
    "output\\output_hw07\\p2_epst10.json",
    "output\\output_hw07\\p2_epst20.json",
    "output\\output_hw07\\p2_epst30.json",
    "output\\output_hw07\\p2_epst40.json",
    "output\\output_hw07\\p2_epst50.json",
    "output\\output_hw07\\p2_et.json",
    "output\\output_hw07\\p2_epsilon.json",
    "output\\output_hw07\\p3_t.json",
    "output\\output_hw07\\p3_e1.json",
    "output\\output_hw07\\p3_e2.json",
    "output\\output_hw07\\p3_e3.json",
    "output\\output_hw07\\p3_e4.json",
    "output\\output_hw07\\p3_h.json",
];

pub fn homework07v2(_rng: &mut ThreadRng) -> () {
    let hw07_start = Instant::now();
    println!("Homework 07v2 start!");

    let run_hw07problem1 = false;
    let run_hw07problem2 = true;
    let run_hw07problem3 = true;
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

    let dt = (1..=8).map(|i| (i as f64) * 0.025).collect::<Vec<f64>>();
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

    let alpha = 0.0;
    let n = 100;
    let m: f64 = 1.0;
    let k: f64 = 1.0;

    let b = 0.8;
    let sigma0 = 10;

    let (x0_i, v0_i) = (0..n)
        .map(|i| {
            (0.0, {
                let a = ((i as f64) - (n as f64) / 2.0) / (sigma0 as f64);
                b * (-a * a).exp()
            })
        })
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

    let eps_i = |x_i: &[f64], v_i: &[f64]| {
        let xv_i = x_i.iter().zip(v_i);
        let imax = xv_i.len() - 1;

        xv_i.enumerate()
            .map(|(i, (&xi, &vi))| {
                let (im1, ip1) = match i {
                    0 => (imax, 1),
                    i if i == imax => (imax - 1, 0),
                    i => (i - 1, i + 1),
                };
                let (xim1, xip1) = (x_i[im1], x_i[ip1]);
                m / 2.0 * vi * vi
                    + k / 4.0
                        * ({
                            let p = xip1 - xi;
                            let q = xi - xim1;
                            p * p + q * q
                        })
            })
            .collect::<Vec<f64>>()
    };

    let dt = 0.04;
    let t_max = 500;
    let t_plot = (0..=5).map(|i| (i as f64) * 10.0).collect::<Vec<f64>>();

    let nt = ((t_max as f64) / dt).round() as usize;
    let t = (0..=nt).map(|t| (t as f64) * dt).collect::<Vec<f64>>();
    write_data_to_file(&t, PATHS[8], FileDataType::Json);

    let mut idx = 0;
    let (et_t, eps_t) = t
        .iter()
        .enumerate()
        .scan(
            (x0_i.clone(), v0_i.clone(), a_i(&x0_i, &v0_i)),
            |(xt_i, vt_i, at_i), (n, tn)| {
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
                let et = m / 2.0 * vt_i.iter().map(|&vti| vti * vti).sum::<f64>()
                    + k / 2.0
                        * xt_i
                            .iter()
                            .enumerate()
                            .map(|(i, xti)| {
                                let im1 = if i == 0 { xt_i.len() - 1 } else { i - 1 };
                                let a = xti - xt_i[im1];
                                a * a
                            })
                            .sum::<f64>();
                let epsi_t = eps_i(xt_i, vt_i);
                if t_plot.contains(tn) {
                    write_data_to_file(&epsi_t, PATHS[9 + idx], FileDataType::Json);
                    idx += 1;
                }
                Some((et, epsi_t.iter().sum::<f64>()))
            },
        )
        .unzip::<f64, f64, Vec<f64>, Vec<f64>>();

    write_data_to_file(&et_t, PATHS[15], FileDataType::Json);
    write_data_to_file(&eps_t, PATHS[16], FileDataType::Json);

    let hw07p2_duration = hw07p2_start.elapsed();
    println!(
        "Hw 07v2 problem 2 finish! Time elapsed: {:?}",
        hw07p2_duration
    );
}

fn hw07_problem3() -> () {
    let hw07p3_start = Instant::now();
    println!("Hw 07v2 problem 3 start!");

    let n = 32;
    let m = 1.0;
    let k = 1.0;
    let alpha = 1.0;
    let dt: f64 = 0.025;
    let tmax: f64 = 1200.0;

    let (x0_i, v0_i) = (0..n)
        .map(|i| ((2.0 * PI * (i as f64) / (n as f64)).cos(), 0.0))
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

    let q_l = |l: usize| 2.0 * PI * (l as f64) / (n as f64);
    let omega_l = |l: usize| 2.0 * (k / m).sqrt() * ((q_l(l) / 2.0).sin()).abs();
    let xic_l_i = |l: usize, i: usize| (2.0 / (n as f64)).sqrt() * (q_l(l) * (i as f64)).cos();
    let xis_l_i = |l: usize, i: usize| (2.0 / (n as f64)).sqrt() * (q_l(l) * (i as f64)).sin();

    let nt = (tmax / dt).round() as usize;
    let t = (0..=nt).map(|i| (i as f64) * dt).collect::<Vec<f64>>();
    let (e1e2e3e4_t, h_t) = t
        .iter()
        .enumerate()
        .scan(
            (x0_i.clone(), v0_i.clone(), a_i(&x0_i, &v0_i)),
            |(xt_i, vt_i, at_i), (n, tn)| {
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
                let e1 = {
                    let qc1w1 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xic_l_i(1, i))
                        .sum::<f64>()
                        * omega_l(1);
                    let qs1w1 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xis_l_i(1, i))
                        .sum::<f64>()
                        * omega_l(1);
                    let qdc1 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(1, i))
                        .sum::<f64>();
                    let qds1 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(1, i))
                        .sum::<f64>();
                    qc1w1 * qc1w1 + qs1w1 * qs1w1 + qdc1 * qdc1 + qds1 * qds1
                };
                let e2 = {
                    let qc2w2 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xic_l_i(2, i))
                        .sum::<f64>()
                        * omega_l(2);
                    let qs2w2 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xis_l_i(2, i))
                        .sum::<f64>()
                        * omega_l(2);
                    let qdc2 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(2, i))
                        .sum::<f64>();
                    let qds2 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(2, i))
                        .sum::<f64>();
                    qc2w2 * qc2w2 + qs2w2 * qs2w2 + qdc2 * qdc2 + qds2 * qds2
                };
                let e3 = {
                    let qc3w3 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xic_l_i(3, i))
                        .sum::<f64>()
                        * omega_l(3);
                    let qs3w3 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xis_l_i(3, i))
                        .sum::<f64>()
                        * omega_l(3);
                    let qdc3 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(3, i))
                        .sum::<f64>();
                    let qds3 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(3, i))
                        .sum::<f64>();
                    qc3w3 * qc3w3 + qs3w3 * qs3w3 + qdc3 * qdc3 + qds3 * qds3
                };
                let e4 = {
                    let qc4w4 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xic_l_i(4, i))
                        .sum::<f64>()
                        * omega_l(4);
                    let qs4w4 = xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| xti * xis_l_i(4, i))
                        .sum::<f64>()
                        * omega_l(4);
                    let qdc4 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(4, i))
                        .sum::<f64>();
                    let qds4 = vt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &vti)| vti * xic_l_i(4, i))
                        .sum::<f64>();
                    qc4w4 * qc4w4 + qs4w4 * qs4w4 + qdc4 * qdc4 + qds4 * qds4
                };
                let h = m / 2.0 * vt_i.iter().map(|&vti| vti * vti).sum::<f64>()
                    + xt_i
                        .iter()
                        .enumerate()
                        .map(|(i, &xti)| {
                            let xip1 = if i == xt_i.len() - 1 {
                                xt_i[0]
                            } else {
                                xt_i[i + 1]
                            };
                            let a = xip1 - xti;
                            k / 2.0 * a * a + alpha / 3.0 * a * a * a
                        })
                        .sum::<f64>();
                Some((((e1, e2), (e3, e4)), h))
            },
        )
        .unzip::<((f64, f64), (f64, f64)), f64, Vec<((f64, f64), (f64, f64))>, Vec<f64>>();

    let (e1e2_t, e3e4_t) = e1e2e3e4_t
        .into_iter()
        .unzip::<(f64, f64), (f64, f64), Vec<(f64, f64)>, Vec<(f64, f64)>>();
    let (e1_t, e2_t) = e1e2_t.into_iter().unzip::<f64, f64, Vec<f64>, Vec<f64>>();
    let (e3_t, e4_t) = e3e4_t.into_iter().unzip::<f64, f64, Vec<f64>, Vec<f64>>();

    write_data_to_file(&t, PATHS[17], FileDataType::Json);
    write_data_to_file(&e1_t, PATHS[18], FileDataType::Json);
    write_data_to_file(&e2_t, PATHS[19], FileDataType::Json);
    write_data_to_file(&e3_t, PATHS[20], FileDataType::Json);
    write_data_to_file(&e4_t, PATHS[21], FileDataType::Json);
    write_data_to_file(&h_t, PATHS[22], FileDataType::Json);

    let hw07p3_duration = hw07p3_start.elapsed();
    println!(
        "Hw 07v2 problem 3 finish! Time elapsed: {:?}",
        hw07p3_duration
    );
}
