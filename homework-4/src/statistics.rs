use crate::config::{q1_config::Q1ParticlePosition, q2_config::Q2ParticlePosition};
use common::statistics::five_point_gauss_legendre_quadrature;

pub fn q1_target_probability_distribution(c: f64) -> impl Fn(Q1ParticlePosition) -> f64 {
    move |x| {
        let a = x.x * x.x - 1.0;
        c * (-4.0 * a * a).exp()
    }
}

pub fn q1_integrate_probability_distribution<T: Fn(f64) -> f64 + Clone>(
    pdf: T,
    lower_bound: f64,
    upper_bound: f64,
    num_steps: usize,
) -> f64 {
    let dx = (upper_bound - lower_bound) / (num_steps as f64);
    (0..num_steps)
        .map(|i| {
            five_point_gauss_legendre_quadrature(
                pdf.clone(),
                (i as f64) * dx,
                ((i + 1) as f64) * dx,
            )
        })
        .sum()
}

pub fn q2_target_probability_distribution(
    lambda: f64,
    width: f64,
) -> impl Fn(Q2ParticlePosition) -> f64 {
    move |x| {
        if (x.x1.abs() < width) && (x.x2.abs() < width) {
            (-lambda / (x.x1 - x.x2).abs()).exp()
        } else {
            0.0
        }
    }
}
