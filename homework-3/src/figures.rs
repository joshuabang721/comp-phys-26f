use comp_phys_common::{
    config::figures_config::ChartParameters,
    figures::{PlotElement, generate_histogram_elements, generate_line_element, generate_plot},
};
use plotters::style::{Color, RGBAColor, colors::*};
use std::error::Error;

pub fn q1_generate_figures(
    dists: Vec<(usize, Vec<f64>)>,
    chart_params: ChartParameters,
    figure_path: String,
) -> Result<(), Box<dyn Error>> {
    let mut range = [(f64::MAX, f64::MIN); 2];
    let plots: Vec<(usize, PlotElement)> = dists
        .iter()
        .map(|x| {
            let e = generate_histogram_elements(x.1.clone(), BLUE.filled());
            let e_range = match e {
                PlotElement::PlotRectangle { corners, .. } => corners,
                PlotElement::PlotLine { range, .. } => range,
                PlotElement::Plot { range, .. } => range,
            };
            range = [
                (range[0].0.min(e_range[0].0), range[0].1.max(e_range[0].1)),
                (range[1].0.min(e_range[1].0), range[1].1.max(e_range[1].1)),
            ];
            (x.0, e)
        })
        .collect();
    let mut i = 1;
    for (n, plot) in plots {
        let fig_path = format!("{}{}.svg", figure_path, i);
        i += 1;
        let params = ChartParameters {
            caption: format!("{}{})", chart_params.caption, n),
            ..chart_params.clone()
        };
        generate_plot(
            vec![
                PlotElement::PlotRectangle {
                    corners: [(range[0].0, range[1].0), (range[0].1, range[1].1)],
                    style: RGBAColor(0, 0, 0, 0.0).filled(),
                },
                plot,
            ],
            &fig_path,
            params,
        )?;
    }
    Ok(())
}

pub fn q2_figures_generate_lineplot(
    sigma: Vec<f64>,
    chart_params: ChartParameters,
    figure_path: String,
) -> Result<(), Box<dyn Error>> {
    let elements = vec![generate_line_element(
        (0..sigma.len()).map(|n| n as f64).collect(),
        sigma,
        BLUE.stroke_width(3),
    )];
    generate_plot(elements, &figure_path, chart_params)?;
    Ok(())
}

pub fn q3_generate_lineplot(
    a: Vec<f64>,
    sigma: Vec<f64>,
    chart_params: ChartParameters,
    figure_path: String,
) -> Result<(), Box<dyn Error>> {
    let elements = vec![generate_line_element(a, sigma, BLUE.stroke_width(3))];
    generate_plot(elements, &figure_path, chart_params)?;
    Ok(())
}

pub fn q4_generate_figures(
    histograms: Vec<(usize, Vec<f64>)>,
    soln: impl Fn(f64, f64) -> f64,
    num_points: usize,
    chart_params: ChartParameters,
    figure_path: String,
) -> Result<(), Box<dyn Error>> {
    let mut range = [(f64::MAX, f64::MIN); 2];
    let elements: Vec<(usize, PlotElement)> = histograms
        .iter()
        .map(|x| {
            let e1: PlotElement = generate_histogram_elements(x.1.clone(), BLUE.filled());
            let e_range = match e1 {
                PlotElement::PlotRectangle { corners, .. } => corners,
                PlotElement::PlotLine { range, .. } => range,
                PlotElement::Plot { range, .. } => range,
            };
            range = [
                (range[0].0.min(e_range[0].0), range[0].1.max(e_range[0].1)),
                (range[1].0.min(e_range[1].0), range[1].1.max(e_range[1].1)),
            ];
            (x.0, e1)
        })
        .collect();

    let mut i = 1;
    let step = (range[0].1 - range[0].0) / (num_points as f64);
    let xr: Vec<f64> = (0..=num_points)
        .map(|x| range[0].0 + (x as f64) * step)
        .collect();

    for (n, plot) in elements {
        let fig_path = format!("{}{}.svg", figure_path, i);
        i += 1;
        let params = ChartParameters {
            caption: format!("{}{})", chart_params.caption, n),
            ..chart_params.clone()
        };
        generate_plot(
            vec![
                PlotElement::PlotRectangle {
                    corners: [(range[0].0, range[1].0), (range[0].1, range[1].1)],
                    style: RGBAColor(0, 0, 0, 0.0).filled(),
                },
                plot,
                PlotElement::PlotLine {
                    points: xr.iter().map(|x| (*x, soln(n as f64, *x))).collect(),
                    range,
                    style: RED.stroke_width(3),
                },
            ],
            &fig_path,
            params,
        )?;
    }

    Ok(())
}
