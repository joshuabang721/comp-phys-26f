use crate::homework7::hw7_question2::Vector3;
use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::{array, ops, time::Instant};

const PATHS: [&str; 12] = [
    "output\\output_hw7\\harmonic_trajectory.json",
    "output\\output_hw7\\harmonic_kinetic.json",
    "output\\output_hw7\\harmonic_potential.json",
    "output\\output_hw7\\harmonic_energy.json",
    "output\\output_hw7\\harmonic_error.json",
    "output\\output_hw7\\chain_x1.json",
    "output\\output_hw7\\chain_x2.json",
    "output\\output_hw7\\chain_x3.json",
    "output\\output_hw7\\chain_kinetic.json",
    "output\\output_hw7\\chain_potential.json",
    "output\\output_hw7\\chain_total.json",
    "output\\output_hw7\\chain_error.json",
];

pub fn homework7(rng: &mut ThreadRng) -> () {
    let hw7_start = Instant::now();
    println!("Homework 7 start!");

    hw7_question1();
    hw7_question2();

    publish_matlab_script(
        "scripts\\matlab_hw7.m",
        "matlab_hw7",
        "output\\output_hw7\\hw7_document.pdf",
        PATHS.to_vec(),
    );

    let hw7_duration = hw7_start.elapsed();
    println!("Homework 7 finish! Time elapsed: {:?}", hw7_duration);
}

fn hw7_question1() -> () {
    let hw7q1_start = Instant::now();
    println!("Hw 7 question 1 start!");

    let m = 1.0;
    let k = 1.0;
    let x0 = 1.0;
    let v0 = 0.0;
    let dt = 0.1;
    let n_max = 300;
    let a = |x: f64| -k * x / m;

    let (x, v) = velocity_verlet(dt, x0, v0, a, n_max);

    write_data_to_file(&x, PATHS[0], FileDataType::Json);

    let e_k = |v: f64| 0.5 * m * v * v;
    let e_p = |x: f64| 0.5 * k * x * x;
    let e_t = |x: f64, v: f64| e_k(v) + e_p(x);

    let ek_n = (0..=n_max).map(|n| e_k(v[n])).collect::<Vec<f64>>();
    let ep_n = (0..=n_max).map(|n| e_p(x[n])).collect::<Vec<f64>>();
    let et_n = (0..=n_max).map(|n| e_t(x[n], v[n])).collect::<Vec<f64>>();

    write_data_to_file(&ek_n, PATHS[1], FileDataType::Json);
    write_data_to_file(&ep_n, PATHS[2], FileDataType::Json);
    write_data_to_file(&et_n, PATHS[3], FileDataType::Json);

    let dt: [f64; 8] = array::from_fn(|i| ((i + 1) as f64) * 0.025);

    let error = dt.map(|dt| {
        let (x, v) = velocity_verlet(dt, x0, v0, a, n_max);
        let e_exact = e_t(x[0], v[0]);
        let et_n = (0..=n_max).map(|n| e_t(x[n], v[n])).collect::<Vec<f64>>();
        et_n.iter()
            .fold(f64::MIN, |acc, x| acc.max((x - e_exact).abs()))
    });

    write_data_to_file(&error, PATHS[4], FileDataType::Json);

    let hw7q1_duration = hw7q1_start.elapsed();
    println!("Hw 7 question 1 finish! Time elapsed: {:?}", hw7q1_duration);
}

fn hw7_question2() -> () {
    let hw7q2_start = Instant::now();
    println!("Hw 7 question 2 start!");

    let m = 1.0;
    let k = 1.0;
    let dt = 0.1;
    let n_max = 300;
    let ax = |xim1: f64, xi: f64, xip1: f64| k / m * (xim1 + xip1 - 2.0 * xi);
    let a = |x: Vector3| Vector3 {
        data: [
            ax(x[2], x[0], x[1]),
            ax(x[0], x[1], x[2]),
            ax(x[1], x[2], x[0]),
        ],
    };

    let x0 = Vector3 {
        data: [1.0, 0.0, -0.5],
    };
    let v0 = Vector3 {
        data: [0.0, 0.0, 0.0],
    };

    let (x, v) = velocity_verlet(dt, x0, v0, a, n_max);

    let (x1, x2, x3) = x.iter().fold(
        (
            Vec::with_capacity(x.len()),
            Vec::with_capacity(x.len()),
            Vec::with_capacity(x.len()),
        ),
        |mut acc, x| {
            acc.0.push(x[0]);
            acc.1.push(x[1]);
            acc.2.push(x[2]);
            acc
        },
    );

    write_data_to_file(&x1, PATHS[5], FileDataType::Json);
    write_data_to_file(&x2, PATHS[6], FileDataType::Json);
    write_data_to_file(&x3, PATHS[7], FileDataType::Json);

    let e_k = |v: Vector3| 0.5 * m * (v[0] + v[1] + v[2]);
    let e_p = |x: Vector3| 0.5 * k * (x[0] + x[1] + x[2]);
    let e_t = |x: Vector3, v: Vector3| e_k(v) + e_p(x);

    let ek_n = (0..=n_max).map(|n| e_k(v[n])).collect::<Vec<f64>>();
    let ep_n = (0..=n_max).map(|n| e_p(x[n])).collect::<Vec<f64>>();
    let et_n = (0..=n_max).map(|n| e_t(x[n], v[n])).collect::<Vec<f64>>();

    write_data_to_file(&ek_n, PATHS[8], FileDataType::Json);
    write_data_to_file(&ep_n, PATHS[9], FileDataType::Json);
    write_data_to_file(&et_n, PATHS[10], FileDataType::Json);

    let dt: [f64; 8] = array::from_fn(|i| ((i + 1) as f64) * 0.025);

    let error = dt.map(|dt| {
        let (x, v) = velocity_verlet(dt, x0, v0, a, n_max);
        let e_exact = e_t(x[0], v[0]);
        let et_n = (0..=n_max).map(|n| e_t(x[n], v[n])).collect::<Vec<f64>>();
        et_n.iter()
            .fold(f64::MIN, |acc, x| acc.max((x - e_exact).abs()))
    });

    write_data_to_file(&error, PATHS[11], FileDataType::Json);

    let hw7q2_duration = hw7q2_start.elapsed();
    println!("Hw 7 question 2 finish! Time elapsed: {:?}", hw7q2_duration);
}

pub(super) mod hw7_question2 {
    use std::ops;

    #[derive(Debug, Clone, Copy)]
    pub(super) struct Vector3 {
        pub data: [f64; 3],
    }

    impl ops::Index<usize> for Vector3 {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl Default for Vector3 {
        fn default() -> Self {
            Self {
                data: [0.0, 0.0, 0.0],
            }
        }
    }

    impl ops::Add for Vector3 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::Output {
                data: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
            }
        }
    }

    impl ops::Mul<f64> for Vector3 {
        type Output = Vector3;
        fn mul(self, rhs: f64) -> Self::Output {
            Self::Output {
                data: [rhs * self[0], rhs * self[1], rhs * self[2]],
            }
        }
    }
}

fn velocity_verlet<T, F>(dt: f64, x0: T, v0: T, a: F, n_max: usize) -> (Vec<T>, Vec<T>)
where
    T: Clone + Copy + Default + ops::Add<Output = T> + ops::Mul<f64, Output = T>,
    F: Fn(T) -> T,
{
    let mut x_n = T::default();
    let mut v_n = T::default();

    (0..=n_max)
        .map(|n| {
            (x_n, v_n) = if n == 0 {
                (x0, v0)
            } else {
                let a_nm1 = a(x_n);
                x_n = x_n + v_n * dt + a_nm1 * 0.5 * dt * dt;
                let a_n = a(x_n);
                v_n = v_n + (a_nm1 + a_n) * 0.5 * dt;
                (x_n, v_n)
            };
            (x_n, v_n)
        })
        .unzip::<T, T, Vec<T>, Vec<T>>()
}
