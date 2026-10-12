use crate::{
    config::q1_config::{Q1Parameters, Q1ParticlePosition},
    figures::q1_generate_figures,
    sampling::sample_markov_chain,
    simulation::simulate_markov_chain,
    statistics::{q1_integrate_probability_distribution, q1_target_probability_distribution},
};
use common::{
    config::chart_config::ChartParameters,
    data_io::{read_data_from_json, write_data_to_json},
};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_1(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q1Parameters {
        coefficient_parameters: cof,
        program_parameters: pgm,
        simulation_parameters: sim,
        distribution_parameters: dst,
        figure_parameters: fig,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let c;
    if pgm.should_calculate_coefficient {
        c = 0.5
            / q1_integrate_probability_distribution(
                |x: f64| q1_target_probability_distribution(1.0)(Q1ParticlePosition { x: x }),
                cof.lower_bound,
                cof.upper_bound,
                cof.num_steps,
            );

        write_data_to_json(pgm.coefficient_value_path, &f64::to_bits(c))?;
    } else {
        c = f64::from_bits(read_data_from_json(pgm.coefficient_value_path)?);
    }

    let markov_chain: Vec<Q1ParticlePosition>;
    if pgm.should_calculate_statistics {
        let markov_chain_samples = sample_markov_chain(
            sim.initial_equilibration_portion + sim.number_of_steps,
            dst.characteristic_step_size,
            rng,
        );
        markov_chain = simulate_markov_chain(
            q1_target_probability_distribution(c),
            Q1ParticlePosition {
                x: dst.initial_position,
            },
            markov_chain_samples,
            sim.initial_equilibration_portion,
        );

        write_data_to_json::<Vec<f64>>(
            pgm.markov_chain_data_path,
            &markov_chain.iter().map(|x| x.x).collect(),
        )?;
    } else {
        markov_chain = read_data_from_json::<Vec<f64>>(pgm.markov_chain_data_path)?
            .iter()
            .map(|&x| Q1ParticlePosition { x: x })
            .collect();
    }

    if pgm.should_generate_figures {
        let chart_params: ChartParameters =
            toml::from_str(&read_to_string(fig.chart_parameters_path)?)?;
        q1_generate_figures(markov_chain, fig.figure_path, chart_params)?;
    }
    Ok(())
}
