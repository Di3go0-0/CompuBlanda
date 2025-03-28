use super::determinant::determinant;

/// Calcula la matriz adjunta (matriz de cofactores)
///
/// # Arguments
/// * `matrix` - Vector de vectores que representa la matriz cuadrada
///
/// # Returns
/// * `Vec<Vec<f64>>` - La matriz adjunta
pub fn adjugate(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = matrix.len();
    if n == 0 || matrix[0].len() != n {
        return vec![];
    }

    // Para una matriz 1x1, la adjunta es [1]
    if n == 1 {
        return vec![vec![1.0]];
    }

    let mut result = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            result[i][j] = cofactor(matrix, i, j);
        }
    }

    result
}

/// Calcula el cofactor para la posición (i,j)
fn cofactor(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> f64 {
    let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
    let minor = get_minor(matrix, row, col);
    sign * determinant(&minor)
}

/// Obtiene la matriz menor eliminando la fila y columna especificadas
fn get_minor(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut minor = Vec::new();

    for i in 0..n {
        if i == row {
            continue;
        }
        let mut new_row = Vec::new();
        for j in 0..n {
            if j == col {
                continue;
            }
            new_row.push(matrix[i][j]);
        }
        minor.push(new_row);
    }

    minor
}
