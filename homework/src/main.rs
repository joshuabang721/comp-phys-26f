use homework::{
    homework1::homework1, homework5::homework5, homework6::homework6, homework7::homework7,
};
use rand::rngs::ThreadRng;
use std::time::Instant;

fn main() -> () {
    let mut rng: ThreadRng = rand::rng();
    let hw_start = Instant::now();
    println!("Homework start!");

    let run_homework1 = false;
    let run_homework2 = false;
    let run_homework3 = false;
    let run_homework4 = false;
    let run_homework5 = false;
    let run_homework6 = false;
    let run_homework7 = false;

    if run_homework1 {
        homework1(&mut rng);
    }
    if run_homework2 {
        // homework2(&mut rng);
    }
    if run_homework3 {
        // homework3(&mut rng);
    }
    if run_homework4 {
        // homework4(&mut rng);
    }
    if run_homework5 {
        homework5(&mut rng);
    }
    if run_homework6 {
        homework6(&mut rng);
    }
    if run_homework7 {
        homework7(&mut rng);
    }

    let hw_duration = hw_start.elapsed();
    println!("Homework finish! Time elapsed: {:?}", hw_duration);
}
