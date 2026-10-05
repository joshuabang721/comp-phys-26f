use crate::{
    config::q3_config::Q3Parameters, figures::q3_generate_lineplot, sampling::sample_random_steps,
    simulation::simulate_random_walks, statistics::q3_statistic_step_sigma,
};
use comp_phys_common::{
    config::figures_config::ChartParameters,
    data_io::{read_data_from_json, write_data_to_json},
};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_3(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q3Parameters {
        simulation_parameters: sim,
        program_parameters: pgm,
        distribution_parameters: dst,
        figures_parameters: fig,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let chart_params: ChartParameters = toml::from_str(&read_to_string(fig.chart_param_path)?)?;

    let da = (dst.step_length_max - dst.step_length_min) / (dst.step_length_num as f64);
    let step_lengths: Vec<f64> = (0..=dst.step_length_num)
        .map(|i| match i {
            0 => dst.step_length_min,
            _ if i == dst.step_length_num => dst.step_length_max,
            _ => da * (i as f64),
        })
        .collect();
    let sigmas: Vec<f64>;

    if pgm.should_generate_statistics {
        let random_step_samples: Vec<Vec<Vec<f64>>> = sample_random_steps(
            sim.number_of_walks,
            sim.number_of_steps,
            &step_lengths,
            dst.random_step_dist_type,
            rng,
        );
        let random_walk_simulation: Vec<Vec<Vec<f64>>> = simulate_random_walks(
            random_step_samples,
            dst.initial_position,
        );
        sigmas = q3_statistic_step_sigma(random_walk_simulation);
        write_data_to_json(pgm.step_data_path, &sigmas)?;
    } else {
        sigmas = read_data_from_json(pgm.step_data_path)?;
    }

    if pgm.should_generate_figures {
        q3_generate_lineplot(step_lengths, sigmas, chart_params, fig.steps_figure_path)?
    }

    Ok(())
}
