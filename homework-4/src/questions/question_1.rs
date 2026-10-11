use crate::{
    config::q1_config::Q1Parameters, figures::q1_generate_figures, sampling::sample_markov_chain,
    simulation::simulate_markov_chain, statistics::q1_target_probability_distribution,
};
use common::{
    config::figures_config::ChartParameters,
    data_io::{read_data_from_json, write_data_to_json},
    statistics::five_point_gauss_legendre_quadrature,
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
        let dx = (cof.upper_bound - cof.lower_bound) / (cof.num_steps as f64);
        let integral: f64 = (0..cof.num_steps)
            .map(|i| {
                five_point_gauss_legendre_quadrature(
                    q1_target_probability_distribution(1.0),
                    (i as f64) * dx,
                    ((i + 1) as f64) * dx,
                )
            })
            .sum();
        c = 0.5 / integral;

        write_data_to_json(pgm.coefficient_value_path, &f64::to_bits(c))?;
    } else {
        c = f64::from_bits(read_data_from_json(pgm.coefficient_value_path)?);
    }

    let markov_chain;
    if pgm.should_calculate_statistics {
        let markov_chain_samples = sample_markov_chain(
            sim.initial_equilibration_portion + sim.number_of_steps,
            dst.characteristic_step_size,
            rng,
        );
        markov_chain = simulate_markov_chain(
            dst.initial_position,
            markov_chain_samples,
            q1_target_probability_distribution(c),
            sim.initial_equilibration_portion,
        );

        write_data_to_json(pgm.markov_chain_data_path, &markov_chain)?;
    } else {
        markov_chain = read_data_from_json(pgm.markov_chain_data_path)?;
    }

    let chart_params: ChartParameters =
        toml::from_str(&read_to_string(fig.chart_parameters_path)?)?;

    if pgm.should_generate_figures {
        q1_generate_figures(markov_chain, fig.figure_path, chart_params)?;
    }
    Ok(())
}
