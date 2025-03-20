// Funciones auxiliares (calculo de distancias)

pub fn centers_distance(x1: &Vec<i32>, x2: &Vec<i32>) -> f64 {
    // Verificamos que los vectores tengan la misma longitud
    assert_eq!(
        x1.len(),
        x2.len(),
        "Los vectores deben tener la misma longitud"
    );

    // Calculamos la suma de las diferencias al cuadrado
    let sum: i32 = x1
        .iter()
        .zip(x2.iter()) // Emparejamos los elementos de ambos vectores
        .map(|(a, b)| (a - b).pow(2)) // Restamos y elevamos al cuadrado
        .sum(); // Sumamos todos los resultados

    // Convertimos a f64 y calculamos la raíz cuadrada
    (sum as f64).sqrt()
}

pub fn e(distance: f64) -> f64 {
    let g: f64 = 0.5;
    // Calculamos e^(-(distance²)/(2g²))
    (-distance.powi(2) / (2.0 * g.powi(2))).exp()
}
