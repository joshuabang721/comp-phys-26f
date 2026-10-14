use crate::homework6::hw6_question1::hw6_question1;
use config::config_hw6::config_hw6::Hw6Parameters;
use dataio::dataio::{FileDataType, read_data_from_file};
use rand::rngs::ThreadRng;

pub fn homework6(params_path: String, rng: &mut ThreadRng) -> () {
    let Hw6Parameters {
        should_run_hw6: run,
        params_paths_hw6,
    } = read_data_from_file(params_path, FileDataType::Toml);

    let run_hw6_questions: [(bool, fn(String, &mut ThreadRng)); 1] = [
        (run.should_run_hw6q1, hw6_question1),
        // (run.should_run_hw6q2, hw6_question2),
        // (run.should_run_hw6q3, hw6_question3),
        // (run.should_run_hw6q4, hw6_question4),
        // (run.should_run_hw6q5, hw6_question5),
        // (run.should_run_hw6q6, hw6_question6),
    ];

    for (i, (run_q, question)) in run_hw6_questions.iter().enumerate() {
        if *run_q {
            question(params_paths_hw6.replace("i", &(i + 1).to_string()), rng);
        }
    }
}
