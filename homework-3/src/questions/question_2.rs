use crate::{
    config::q2_config::Q2Parameters, figures::q2_figures_generate_lineplot,
    sampling::sample_random_steps, simulation::simulate_random_walks,
    statistics::q2_statistic_walk_sigma,
};
use comp_phys_common::{
    config::figures_config::ChartParameters,
    data_io::{read_data_from_json, write_data_to_json},
};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_2(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q2Parameters {
        simulation_parameters: sim,
        program_parameters: pgm,
        distribution_parameters: dst,
        figures_parameters: fig,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let chart_params: ChartParameters = toml::from_str(&read_to_string(fig.chart_param_path)?)?;

    let sigma: Vec<f64>;
    if pgm.should_generate_statistics {
        let random_step_samples: Vec<Vec<Vec<f64>>> = sample_random_steps(
            sim.number_of_walks,
            sim.number_of_steps,
            &vec![dst.step_length],
            dst.random_step_dist_type,
            rng,
        );
        let random_walk_simulation: Vec<Vec<Vec<f64>>> = simulate_random_walks(
            random_step_samples,
            dst.initial_position,
        );
        sigma = q2_statistic_walk_sigma(&random_walk_simulation[0]);
        write_data_to_json(pgm.sigma_data_path, &sigma)?;
    } else {
        sigma = read_data_from_json(pgm.sigma_data_path)?;
    }

    if pgm.should_generate_figures {
        q2_figures_generate_lineplot(sigma, chart_params, fig.histogram_figure_path)?;
    }

    Ok(())
}
