use crate::config::{hw4_config::target_probability_distribution, q0_config::Q0Parameters};
use comp_phys_common::statistics::five_point_gauss_legendre_quadrature;
use std::{error::Error, fs::read_to_string};

pub fn question_0(params_path: String) -> Result<(), Box<dyn Error>> {
    let params: Q0Parameters = toml::from_str(&read_to_string(params_path)?)?;

    let pdf = target_probability_distribution(1.0);

    let sine = |x: f64| x.cos();

    println!("{}", five_point_gauss_legendre_quadrature(sine));

    // // let mut x   = 0x3FE234567899847F;
    // let mut x = 4603299315121306847;
    // let mut i = 0;
    // loop {
    //     let y = func(x);
    //     println!(
    //         "x u64 = {}\nx f64 = {}\nf(x) f64 = {}\nf(x) u64 = {}\n",
    //         x,
    //         f64::from_bits(x),
    //         y,
    //         f64::to_bits(y)
    //     );
    //     if y > 0.0 {
    //         break;
    //     }
    //     x += 1;
    //     i += 1;
    //     if i > 1000 {
    //         println!("Nope :(");
    //         break;
    //     }
    // }

    Ok(())
}
