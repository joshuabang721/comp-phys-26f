use crate::config::figures_config::ChartParameters;
use plotters::{
    backend::{DrawingBackend, SVGBackend},
    chart::{ChartBuilder, ChartContext},
    coord::{cartesian::Cartesian2d, types::RangedCoordf64},
    drawing::IntoDrawingArea,
    element::{PathElement, Rectangle},
    style::{RGBAColor, ShapeStyle, colors::*},
};
use std::{error::Error, iter};

pub fn generate_histogram_elements(data: Vec<f64>, style: ShapeStyle) -> PlotElement {
    let num_bins: usize = data.len().ilog2() as usize;
    let range_x = data.iter().fold((0.0, 0.0), |acc: (f64, f64), &x: &f64| {
        (acc.0.min(x), acc.1.max(x))
    });

    let step = (range_x.1 - range_x.0) / (num_bins as f64);
    let c: f64 = 1.0 / ((data.len() as f64) * step);
    let mut max_density = f64::MIN;
    let p_density: Vec<f64> = data
        .iter()
        .fold(vec![0usize; num_bins], |mut acc, &x| {
            let i = if x <= range_x.0 {
                0
            } else if x >= range_x.1 {
                num_bins - 1
            } else {
                ((x - range_x.0) / step) as usize
            };
            acc[i] += 1;
            acc
        })
        .iter()
        .map(|&x| {
            let density: f64 = c * (x as f64);
            max_density = max_density.max(density);
            density
        })
        .collect();

    let rectangles = {
        (0..(p_density.len()))
            .map(|i| PlotElement::PlotRectangle {
                corners: [
                    (
                        if i == 0 {
                            range_x.0
                        } else {
                            range_x.0 + step * (i as f64)
                        },
                        0.0,
                    ),
                    (
                        if i == p_density.len() - 1 {
                            range_x.1
                        } else {
                            range_x.0 + step * ((i + 1) as f64)
                        },
                        p_density[i] as f64,
                    ),
                ],
                style: style,
            })
            .collect()
    };
    PlotElement::Plot {
        elements: rectangles,
        range: ([range_x, (0.0, max_density)]),
    }
}

pub fn generate_line_element(data_x: Vec<f64>, data_y: Vec<f64>, style: ShapeStyle) -> PlotElement {
    let mut range = [(f64::MAX, f64::MIN), (f64::MAX, f64::MIN)];
    let points: Vec<(f64, f64)> = (0..data_x.len().min(data_y.len()))
        .map(|i| {
            let point = (data_x[i], data_y[i]);
            range = [
                (range[0].0.min(point.0), range[0].1.max(point.0)),
                (range[1].0.min(point.1), range[1].1.max(point.1)),
            ];
            point
        })
        .collect();

    PlotElement::PlotLine {
        points,
        range,
        style,
    }
}

pub fn generate_plot(
    elements: Vec<PlotElement>,
    file_path: &str,
    params: ChartParameters,
) -> Result<(), Box<dyn Error>> {
    let font = params.font.as_str();

    let mut e_num: usize = 0;
    let range = (&elements)
        .iter()
        .fold([(0.0, 0.0); 2], |acc: [(f64, f64); 2], e| {
            e_num += 1;
            let e_range = match e {
                PlotElement::PlotRectangle { corners, .. } => &[
                    (
                        corners[0].0.min(corners[1].0),
                        corners[0].0.max(corners[1].0),
                    ),
                    (
                        corners[0].1.min(corners[1].1),
                        corners[0].1.max(corners[1].1),
                    ),
                ],
                PlotElement::PlotLine { range, .. } => range,
                PlotElement::Plot { range, .. } => range,
            };
            [
                (
                    acc[0].0.min(e_range[0].0).max(-1000.0),
                    acc[0].1.max(e_range[0].1).min(1000.0),
                ),
                (
                    acc[1].0.min(e_range[1].0).max(-1000.0),
                    acc[1].1.max(e_range[1].1).min(1000.0),
                ),
            ]
        });

    let chart_padding = (
        params.chart_padding * (range[0].1 - range[0].0),
        params.chart_padding * (range[1].1 - range[1].0),
    );

    let (r, g, b, a) = params.chart_color;
    let chart_color = RGBAColor(r, g, b, a);
    let drawing_area = SVGBackend::new(&file_path, (800, 600)).into_drawing_area();
    drawing_area.fill(&chart_color)?;

    let mut chart_builder = ChartBuilder::on(&drawing_area);
    let mut chart_context = chart_builder
        .margin(params.margin)
        .x_label_area_size(params.x_label_area_size)
        .y_label_area_size(params.y_label_area_size)
        .caption(params.caption.as_str(), (font, params.caption_size))
        .build_cartesian_2d(
            (range[0].0 - chart_padding.0)..(range[0].1 + chart_padding.0),
            (range[1].0 - chart_padding.1)..(range[1].1 + chart_padding.1),
        )?;

    chart_context
        .configure_mesh()
        .max_light_lines(params.max_light_lines)
        .x_label_style((font, params.label_size))
        .y_label_style((font, params.label_size))
        .x_desc(params.x_desc.as_str())
        .y_desc(params.y_desc.as_str())
        .axis_desc_style((font, params.desc_size))
        .draw()?;

    chart_context.draw_series([
        PathElement::new(
            [
                (range[0].0 - chart_padding.0, 0.0),
                (range[0].1 + chart_padding.0, 0.0),
            ],
            ShapeStyle::from(&BLACK).stroke_width(3),
        ),
        PathElement::new(
            [
                (0.0, range[1].0 - chart_padding.1),
                (0.0, range[1].1 + chart_padding.1),
            ],
            ShapeStyle::from(&BLACK).stroke_width(3),
        ),
    ])?;

    for element in elements {
        draw_plot_element(&mut chart_context, &element)?;
    }

    drawing_area.present()?;
    Ok(())
}

pub enum PlotElement {
    PlotRectangle {
        corners: [(f64, f64); 2],
        style: ShapeStyle,
    },
    PlotLine {
        points: Vec<(f64, f64)>,
        range: [(f64, f64); 2],
        style: ShapeStyle,
    },
    Plot {
        elements: Vec<PlotElement>,
        range: [(f64, f64); 2],
    },
}

fn draw_plot_element<DB>(
    chart_context: &mut ChartContext<DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    element: &PlotElement,
) -> Result<(), Box<dyn Error>>
where
    DB: DrawingBackend,
    DB::ErrorType: 'static,
{
    match element {
        PlotElement::PlotRectangle { corners, style } => {
            chart_context.draw_series(iter::once(Rectangle::new(*corners, *style)))?;
        }
        PlotElement::PlotLine { points, style, .. } => {
            chart_context.draw_series(iter::once(PathElement::new(points.clone(), *style)))?;
        }
        PlotElement::Plot { elements, .. } => {
            for e in elements {
                draw_plot_element(chart_context, e)?;
            }
        }
    }
    Ok(())
}
