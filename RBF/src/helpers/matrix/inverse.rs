use super::{adjugate::adjugate, determinant::determinant, transpose::transpose};

/// Calcula la inversa de una matriz usando la fórmula: A⁻¹ = adj(Aᵀ)/det(A)
///
/// # Arguments
/// * `matrix` - Matriz de entrada como Vec<Vec<f64>>
///
/// # Returns
/// * `Option<Vec<Vec<f64>>>` - Some(matriz_inversa) si existe, None si no existe
pub fn inverse(matrix: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = matrix.len();

    // Verificamos que la matriz sea cuadrada
    if n == 0 || matrix[0].len() != n {
        return None;
    }

    // Calculamos el determinante
    let det = determinant(matrix);

    // Si el determinante es 0, la matriz no tiene inversa
    if det == 0.0 {
        return None;
    }

    // 1. Calculamos la transpuesta
    let transposed = transpose(matrix);

    // 2. Calculamos la adjunta de la transpuesta
    let adj_trans = adjugate(&transposed);

    // 3. Dividimos por el determinante
    let mut inverse = vec![vec![0.0; n]; n];
    let inv_det = 1.0 / det;

    for i in 0..n {
        for j in 0..n {
            inverse[i][j] = adj_trans[i][j] * inv_det;
        }
    }

    Some(inverse)
}
