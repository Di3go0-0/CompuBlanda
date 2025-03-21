use super::gaussiana::{centers::get_center, centers_distance::centers_distance, e::e};

/// Realiza la clasificación usando RBF
///
/// # Arguments
/// * `w` - Vector de pesos
/// * `center` - Centro actual para el cual queremos predecir
/// * `x1` - Vector de entrenamiento x1
/// * `x2` - Vector de entrenamiento x2
///
/// # Returns
/// * `f64` - Predicción calculada
pub fn clasification(w: &Vec<f64>, center: &Vec<f64>, x1: &Vec<f64>, x2: &Vec<f64>) -> f64 {
    let n = w.len();

    let mut row: Vec<f64> = Vec::with_capacity(n);

    for i in 0..n {
        if let Some(current_center) = get_center(x1, x2, i) {
            let distance = centers_distance(&center, &current_center);
            let value = e(&distance, x1, x2); // Propaga el error si ocurre
            row.push(value);
        }
    }

    // Calculamos la predicción multiplicando cada peso por su valor correspondiente
    let mut prediction: f64 = 0.0; // Inicializamos en 0
    for i in 0..n {
        prediction += w[i] * row[i];
    }

    prediction // En Rust el return es implícito
}
