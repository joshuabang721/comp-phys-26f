use homework::{homework1::homework1, homework5::homework5, homework7::homework7};
use rand::rngs::ThreadRng;
use std::time::Instant;

fn main() -> () {
    let mut rng: ThreadRng = rand::rng();
    let hw_start = Instant::now();
    println!("Homework start!");

    // homework1(&mut rng);
    // homework2(&mut rng);
    // homework3(&mut rng);
    // homework4(&mut rng);
    // homework5(&mut rng);
    // homework6(&mut rng);
    homework7(&mut rng);

    let hw_duration = hw_start.elapsed();
    println!("Homework finish! Time elapsed: {:?}", hw_duration);
}
