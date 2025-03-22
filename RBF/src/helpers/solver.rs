// Resolucion de ecuacion AW = B
//

use std::f64;

use super::activation;
use super::matrix;

pub fn weight_vector(
    x1: &Vec<f64>,
    x2: &Vec<f64>,
    _f: &Vec<f64>,
    centers: &Vec<Vec<f64>>,
) -> Vec<f64> {
    let activation_matrix = activation::build_activation_matrix(x1, x2, centers);

    let f_to_matrix = matrix::vector_to_matrix::vector_to_matrix(_f);

    let inverse_matrix = match matrix::inverse::inverse(&activation_matrix) {
        Some(matrix) => matrix,
        None => return Vec::new(), // Si no hay inversa, retornamos un vector vacío
    };

    let weight_matrix = match matrix::multiplication::multiply(&inverse_matrix, &f_to_matrix) {
        Some(matrix) => matrix,
        None => return Vec::new(), // Si la multiplicación falla, retornamos un vector vacío
    };

    // Convertimos la matriz resultante a un vector de f64
    weight_matrix.into_iter().flatten().collect()
}
