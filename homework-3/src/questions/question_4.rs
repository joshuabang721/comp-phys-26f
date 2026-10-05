use crate::{
    config::{
        q1_config::Q1Parameters,
        q4_config::Q4Parameters,
    },
    figures::q4_generate_figures,
    statistics::q4_statistic_anal_soln,
};
use comp_phys_common::{config::figures_config::ChartParameters, data_io::read_data_from_json};
use std::{error::Error, fs::read_to_string};

pub fn question_4(params_path: String) -> Result<(), Box<dyn Error>> {
    let Q4Parameters {
        program_parameters: pgm,
        figures_parameters: fig,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let chart_params: ChartParameters = toml::from_str(&read_to_string(fig.chart_param_path)?)?;

    if pgm.should_generate_figures {
        let Q1Parameters {
            distribution_parameters: dst,
            ..
        } = toml::from_str(&read_to_string(pgm.mean_param_path)?)?;
        let q1_histograms: Vec<(usize, Vec<f64>)> = read_data_from_json(pgm.mean_data_path)?;

        let analytical_soln = q4_statistic_anal_soln(
            dst.initial_position,
            dst.step_length,
            dst.random_step_dist_type,
        );

        q4_generate_figures(
            q1_histograms,
            analytical_soln,
            fig.num_points,
            chart_params,
            fig.mixed_figure_path,
        )?;
    }

    Ok(())
}
