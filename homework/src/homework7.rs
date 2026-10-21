use dataio::dataio::{FileDataType, publish_matlab_script, write_data_to_file};
use rand::rngs::ThreadRng;
use std::time::Instant;

const PATHS: [&str; 3] = [
    "output\\output_hw7\\data.json",
    "output\\output_hw7\\data.json",
    "output\\output_hw7\\data.json",
];

pub fn homework7(rng: &mut ThreadRng) -> () {
    let hw7_start = Instant::now();
    println!("Homework 7 start!");

    hw7_question1();

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

    // stuff presumably

    // write_data_to_file(&data, PATHS[0], FileDataType::Json);

    let hw7q1_duration = hw7q1_start.elapsed();
    println!("Hw 7 question 1 finish! Time elapsed: {:?}", hw7q1_duration);
}
