use super::gaussiana::{
    self, centers::get_center, centers_distance::centers_distance, e::e, sigma::calculate_sigma,
};

/// Construye la matriz de activación A para una Red Neuronal RBF
///
/// # Arguments
/// * `x1` - Vector de coordenadas x1 de los puntos
/// * `x2` - Vector de coordenadas x2 de los puntos
/// * `f` - Vector de valores conocidos
///
/// # Returns
/// * `Vec<Vec<f64>>` - Matriz de activación A
pub fn build_activation_matrix(
    x1: &Vec<f64>,
    x2: &Vec<f64>,
    centers: &Vec<Vec<f64>>,
) -> Vec<Vec<f64>> {
    let n = x1.len();
    let mut activation_matrix = Vec::with_capacity(n);
    let g: f64 = calculate_sigma(x1, x2, centers);

    // Para cada punto (x1[i], x2[i])
    for i in 0..n {
        let mut row: Vec<f64> = Vec::with_capacity(n);

        // Obtenemos el punto actual para el cual calcularemos las distancias
        let current_point = vec![x1[i], x2[i]];

        // Calculamos la distancia y la gaussiana con respecto a cada centro
        for j in 0..n {
            if let Some(center) = get_center(x1, x2, j) {
                // Calculamos la distancia entre el punto actual y el centro
                let distance = centers_distance(&current_point, &center);
                // Calculamos el valor de la función gaussiana
                let activation = e(&distance, x1, x2, g);
                row.push(activation);
            }
        }

        activation_matrix.push(row);
    }

    return activation_matrix;
}
