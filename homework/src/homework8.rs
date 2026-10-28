use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::time::Instant;

const PATHS: [&str; 2] = [
    "output\\output_hw8\\lax_wave_times.json",
    "output\\output_hw8\\lax_waveforms.json",
];

pub fn homework8(_rng: &mut ThreadRng) -> () {
    let hw8_start = Instant::now();
    println!("Homework 8 start!");

    let run_hw8question1 = false;
    let run_hw8question2 = false;
    let run_hw8matlab = true;

    if run_hw8question1 {
        hw8_question1();
    }
    if run_hw8question2 {
        // hw8_question2(rng);
    }
    if run_hw8matlab {
        publish_matlab_script(
            "scripts\\matlab_hw8.m",
            "matlab_hw8",
            "output\\output_hw8\\hw8_document.pdf",
            PATHS.to_vec(),
        );
    }

    let hw8_duration = hw8_start.elapsed();
    println!("Homework 8 finish! Time elapsed: {:?}", hw8_duration);
}

fn hw8_question1() -> () {
    let hw8q1_start = Instant::now();
    println!("Hw 8 question 1 start!");

    let v = 1.0;
    let dx = 0.1;
    let w = 5.0;
    let u_x_0 = |x: f64| (-x * x / (w * w)).exp();
    let x_min: f64 = -20.0;
    let x_max: f64 = 50.0;
    let x_num = ((x_max - x_min).abs() / dx) as usize;
    let t_max = 20.0;

    let dt = [0.06, 0.10, 0.14];
    let (t, u_t_j) = dt
        .map(|d| {
            let n_max = (t_max / d) as usize;
            (
                (0..n_max).map(|n| d * (n as f64)).collect::<Vec<f64>>(),
                (0..n_max)
                    .scan(Vec::new(), |state, n| {
                        if n == 0 {
                            Some((0..x_num).map(|x| u_x_0(x as f64)).collect::<Vec<f64>>())
                        } else {
                            Some(lax_update(state, v, d, dx))
                        }
                    })
                    .collect::<Vec<Vec<f64>>>(),
            )
        })
        .into_iter()
        .unzip::<Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<Vec<f64>>>>();
    write_data_to_file(&t, PATHS[0], FileDataType::Json);
    write_data_to_file(&u_t_j, PATHS[1], FileDataType::Json);

    //  Periodic BC: u_50.0_t = u_-20.0_t

    // write_data_to_file(&error, PATHS[4], FileDataType::Json);

    let hw8q1_duration = hw8q1_start.elapsed();
    println!("Hw 8 question 1 finish! Time elapsed: {:?}", hw8q1_duration);
}

fn hw8_question2() -> () {
    let hw8q2_start = Instant::now();
    println!("Hw 8 question 2 start!");

    // write_data_to_file(&error, PATHS[4], FileDataType::Json);

    let hw8q2_duration = hw8q2_start.elapsed();
    println!("Hw 8 question 2 finish! Time elapsed: {:?}", hw8q2_duration);
}

fn lax_update(u_n_j: &Vec<f64>, v: f64, dt: f64, dx: f64) -> Vec<f64> {
    let j_max = u_n_j.len();
    let k = 0.5 * v * dt / dx;
    let c1 = 0.5 - k;
    let c2 = 0.5 + k;
    (0..j_max)
        .map(|j| match j {
            0 => c1 * u_n_j[j + 1] + c2 * u_n_j[j_max - 1],
            j if j == (j_max - 1) => c1 * u_n_j[0] + c2 * u_n_j[j - 1],
            j => c1 * u_n_j[j + 1] + c2 * u_n_j[j - 1],
        })
        .collect::<Vec<f64>>()
}
