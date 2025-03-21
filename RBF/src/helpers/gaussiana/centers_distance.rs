pub fn centers_distance(x1: &Vec<f64>, x2: &Vec<f64>) -> f64 {
    // Verificamos que los vectores tengan la misma longitud
    assert_eq!(
        x1.len(),
        x2.len(),
        "Los vectores deben tener la misma longitud"
    );

    // Calculamos la suma de las diferencias al cuadrado
    let sum: f64 = x1
        .iter()
        .zip(x2.iter()) // Emparejamos los elementos de ambos vectores
        .map(|(a, b)| (a - b) * (a - b)) // Restamos y multiplicamos (equivalente a pow(2))
        .sum(); // Sumamos todos los resultados

    // Calculamos la raíz cuadrada
    sum.sqrt()
}
