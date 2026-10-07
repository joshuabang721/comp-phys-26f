pub fn five_point_gauss_legendre_quadrature<T: Fn(f64) -> f64>(f: T) -> f64 {
    let c: Vec<(u64, u64)> = vec![
        (0x3FCE539EC36E038C, 0xBFECFF6CE0533A69),
        (0x3FDEA1DA25AE415B, 0xBFE13B23FD99B705),
        (0x3FE23456789ABCDF, 0x0),
        (0x3FDEA1DA25AE415B, 0x3FE13B23FD99B705),
        (0x3FCE539EC36E038C, 0x3FECFF6CE0533A69),
    ];

    c.into_iter()
        .map(|n| f64::from_bits(n.0) * f(f64::from_bits(n.1)))
        .sum()
}
