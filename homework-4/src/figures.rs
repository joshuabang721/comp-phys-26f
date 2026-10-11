use common::{
    config::figures_config::ChartParameters,
    figures::{generate_normalized_histogram_elements, generate_plot},
};
use plotters::style::{Color, colors::*};
use std::error::Error;

pub fn q1_generate_figures(
    markov_chain: Vec<f64>,
    file_path: String,
    params: ChartParameters,
) -> Result<(), Box<dyn Error>> {
    let elements = vec![generate_normalized_histogram_elements(
        markov_chain,
        BLUE.filled(),
    )];

    generate_plot(elements, &file_path, params)?;
    Ok(())
}
