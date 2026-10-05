use crate::{
    config::q1_config::Q1Parameters, figures::q1_generate_figures,
    sampling::sample_random_steps, simulation::simulate_random_walks,
    statistics::q1_statistic_walk_dists,
};
use comp_phys_common::{
    config::figures_config::ChartParameters,
    data_io::{read_data_from_json, write_data_to_json},
};
use rand::rngs::ThreadRng;
use std::{error::Error, fs::read_to_string};

pub fn question_1(params_path: String, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
    let Q1Parameters {
        simulation_parameters: sim,
        program_parameters: pgm,
        distribution_parameters: dst,
        figures_parameters: fig,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let chart_params: ChartParameters = toml::from_str(&read_to_string(fig.chart_param_path)?)?;

    let dists: Vec<(usize, Vec<f64>)>;

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
        dists = q1_statistic_walk_dists(
            &random_walk_simulation[0],
            fig.number_of_times,
        );
        write_data_to_json(pgm.mean_data_path, &dists)?;
    } else {
        dists = read_data_from_json(pgm.mean_data_path)?;
    }

    if pgm.should_generate_figures {
        q1_generate_figures(dists, chart_params, fig.histogram_figure_path)?;
    }

    Ok(())
}
