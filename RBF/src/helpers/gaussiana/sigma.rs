use super::{centers::get_center, centers_distance::centers_distance};

/// Calcula el valor de sigma (ancho de la gaussiana) basado en el promedio de las distancias entre centros
///
/// # Arguments
/// * `x1` - Vector de coordenadas x1
/// * `x2` - Vector de coordenadas x2
///
/// # Returns
/// * `Result<f64, String>` - El valor de sigma o un error si algo falla
///
/// # Example
/// ```
/// let x1 = vec![0.0, 0.0, 1.0, 1.0];
/// let x2 = vec![0.0, 1.0, 0.0, 1.0];
/// let sigma = calculate_sigma(&x1, &x2).unwrap();
/// ```
pub fn calculate_sigma(x1: &Vec<f64>, x2: &Vec<f64>) -> f64 {
    let n = x1.len();

    // Validación de entrada
    if n != x2.len() || n == 0 {
        return 1.0; // Valor por defecto en caso de error
    }

    let mut total_distance = 0.0;
    let mut count = 0;

    for i in 0..n {
        let Some(center_i) = get_center(x1, x2, i) else {
            return 1.0;
        };

        for j in (i + 1)..n {
            let Some(center_j) = get_center(x1, x2, j) else {
                return 1.0;
            };

            total_distance += centers_distance(&center_i, &center_j);
            count += 1;
        }
    }

    if count == 0 {
        1.0 // Evitar división por cero
    } else {
        total_distance / count as f64
    }
}
