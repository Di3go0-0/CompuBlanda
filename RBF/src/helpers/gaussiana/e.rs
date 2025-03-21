use super::sigma::calculate_sigma;

pub fn e(distance: &f64, x1: &Vec<f64>, x2: &Vec<f64>) -> f64 {
    let g = calculate_sigma(x1, x2); // Desempaquetamos `Result<f64, String>` usando `?`

    let distance_squared = distance.powi(2);
    let sigma_squared = g.powi(2);

    return ((-distance_squared / (2.0 * sigma_squared)).exp()); // Retornamos el resultado como `Result<f64, String>`
}
