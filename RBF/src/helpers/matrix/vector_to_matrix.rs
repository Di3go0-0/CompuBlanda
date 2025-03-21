/// let vector = vec![1.0, 2.0, 3.0];
/// let matrix = vector_to_matrix(&vector);
/// assert_eq!(matrix, vec![vec![1.0], vec![2.0], vec![3.0]]);
/// ```
pub fn vector_to_matrix(vector: &Vec<f64>) -> Vec<Vec<f64>> {
    // Convertimos cada elemento del vector en una fila con un solo elemento
    vector.iter().map(|&x| vec![x]).collect()
}
