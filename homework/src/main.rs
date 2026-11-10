use homework::{
    homework01::homework1, homework02::homework2, homework03::homework3, homework04::homework4,
    homework05::homework5, homework06::homework6, homework07::homework7, homework08::homework8,
    homework09::homework09, homework10::homework10,
};
use rand::rngs::ThreadRng;
use std::time::Instant;

fn main() -> () {
    let mut rng: ThreadRng = rand::rng();
    let hw_start = Instant::now();
    println!("Homework start!");

    let run_homework01 = false;
    let run_homework02 = false;
    let run_homework03 = false;
    let run_homework04 = false;
    let run_homework05 = false;
    let run_homework06 = true;
    let run_homework07 = false;
    let run_homework08 = false;
    let run_homework09 = false;
    let run_homework10 = false;

    if run_homework01 {
        homework1(&mut rng);
    }
    if run_homework02 {
        homework2(&mut rng);
    }
    if run_homework03 {
        homework3(&mut rng);
    }
    if run_homework04 {
        homework4(&mut rng);
    }
    if run_homework05 {
        homework5(&mut rng);
    }
    if run_homework06 {
        homework6(&mut rng);
    }
    if run_homework07 {
        homework7(&mut rng);
    }
    if run_homework08 {
        homework8(&mut rng);
    }
    if run_homework09 {
        homework09(&mut rng);
    }
    if run_homework10 {
        homework10(&mut rng);
    }

    let hw_duration = hw_start.elapsed();
    println!("Homework finish! Time elapsed: {:?}", hw_duration);
}
