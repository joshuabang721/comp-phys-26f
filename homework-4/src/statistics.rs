pub fn q1_target_probability_distribution(c: f64) -> impl Fn(f64) -> f64 {
    move |x| c * (-4.0 * (x * x - 1.0) * (x * x - 1.0)).exp()
}
