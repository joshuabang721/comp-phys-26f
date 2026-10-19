use homework::homework1::homework1;
use rand::rngs::ThreadRng;

fn main() -> () {
    // let _ = Command::new("matlab")
    //     .current_dir(r"C:\Users\joshu\OneDrive\Documents\Rust Projects\comp-physics")
    //     .args(["-batch", "run('matlab\\matlab_hw2.m')"])
    //     .status();

    // return;

    let mut rng: ThreadRng = rand::rng();

    homework1(&mut rng);

    // let MainParameters {
    //     should_run_hw: run,
    //     params_paths_hw,
    // } = read_data_from_file(PARAMETERS_PATH, FileDataType::Toml);

    // let run_homeworks: [(bool, fn(&str, &mut ThreadRng)); 7] = [
    //     (run.should_run_hw1, homework1),
    //     (run.should_run_hw2, homework2),
    //     (run.should_run_hw3, homework3),
    //     (run.should_run_hw4, homework4),
    //     (run.should_run_hw5, homework5),
    //     (run.should_run_hw6, homework6),
    //     (run.should_run_hw7, homework7),
    // ];

    // for (i, (run_hw, homework)) in run_homeworks.iter().enumerate() {
    //     if *run_hw {
    //         homework(
    //             &params_paths_hw.replace("i", &(i + 1).to_string()),
    //             &mut rng,
    //         );
    //     }
    // }
    println!("Finished!");
}
