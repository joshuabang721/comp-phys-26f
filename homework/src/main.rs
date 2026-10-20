use homework::homework1::homework1;
use rand::rngs::ThreadRng;
// use std::process::Command;

fn main() -> () {
    let mut rng: ThreadRng = rand::rng();

    homework1(&mut rng);

    println!("Finished!");
}
