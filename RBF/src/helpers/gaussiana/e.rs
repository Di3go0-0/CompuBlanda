pub fn e(distance: f64) -> f64 {
    let g: f64 = 0.5;
    // Calculamos e^(-(distance²)/(2g²))
    (-distance.powi(2) / (2.0 * g.powi(2))).exp()
}
