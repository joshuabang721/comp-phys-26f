use crate::{
    config::q2_config::{Q2Parameters, Q2ParticlePosition},
    sampling::sample_markov_chain,
    simulation::simulate_markov_chain,
    statistics::q2_target_probability_distribution,
};
use common::data_io::write_data_to_json;
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_2(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q2Parameters {
        program_parameters: pgm,
        simulation_parameters: sim,
        distribution_parameters: dst,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let heat_map: Vec<Q2ParticlePosition>;
    if pgm.should_calculate_statistics {
        let markov_chain_samples: Vec<(Q2ParticlePosition, _)> = sample_markov_chain(
            sim.initial_equilibration_portion + sim.number_of_steps,
            dst.characteristic_step_size,
            rng,
        );
        heat_map = simulate_markov_chain(
            q2_target_probability_distribution(1.0, dst.box_width),
            Q2ParticlePosition {
                x1: dst.initial_position.0,
                x2: dst.initial_position.1,
            },
            markov_chain_samples,
            sim.initial_equilibration_portion,
        );

        write_data_to_json::<Vec<[f64; 2]>>(
            pgm.heat_map_data_path,
            &heat_map.iter().map(|x| [x.x1, x.x2]).collect(),
        )?;
    }

    Ok(())
}
