use crate::config::{hw4_config::target_probability_distribution, q0_config::Q0Parameters};
use comp_phys_common::{
    data_io::write_data_to_json, statistics::five_point_gauss_legendre_quadrature,
};
use std::{error::Error, fs::read_to_string};

pub fn question_0(params_path: String) -> Result<(), Box<dyn Error>> {
    let Q0Parameters {
        coeff_value_path,
        lower_bound,
        upper_bound,
        num_steps,
    } = toml::from_str(&read_to_string(params_path)?)?;

    let dx = (upper_bound - lower_bound) / (num_steps as f64);
    let integral: f64 = (0..num_steps)
        .map(|i| {
            five_point_gauss_legendre_quadrature(
                target_probability_distribution(1.0),
                (i as f64) * dx,
                ((i + 1) as f64) * dx,
            )
        })
        .sum();

    write_data_to_json(coeff_value_path, &f64::to_bits(0.5 / integral))?;
    Ok(())
}
