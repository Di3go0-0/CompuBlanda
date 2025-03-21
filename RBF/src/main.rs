mod helpers;

fn main() {
    let v1 = vec![0.0, 0.0, 1.0, 1.0];
    let v2 = vec![0.0, 1.0, 0.0, 1.0];

    let distancia = helpers::gaussiana::centers_distance::centers_distance(&v1, &v2);
    println!("La distancia es: {}", distancia);
    let e = helpers::gaussiana::e::e(distancia);
    println!("e: {}", e);
    let c = helpers::matrix::vector_to_matrix::vector_to_matrix(&v1);
    println!("c: {:?}", c);

    if let Some(centro) = helpers::centers::get_center(&v1, &v2, 1) {
        println!("Centro en iteración {}: {:?}", 1, centro);
    }

    let matriz = vec![
        vec![2.0, -2.0, 2.0],
        vec![2.0, 1.0, 0.0],
        vec![3.0, -2.0, 2.0],
    ];
    let det = helpers::matrix::determinant::determinant(&matriz);
    println!("Determinante : {}", det);
    let transpose = helpers::matrix::transpose::transpose(&matriz);
    println!("transpose: {:?}", transpose);
    let adjunta = helpers::matrix::adjugate::adjugate(&transpose);
    println!("adjugate: {:?}", adjunta);
    let inverse = helpers::matrix::inverse::inverse(&matriz);
    println!("inverse: {:?}", inverse);

    let matriz2 = vec![
        vec![1.0, 2.0, 2.0],
        vec![2.0, 1.0, 2.0],
        vec![2.0, 2.0, 1.0],
    ];
    let matriz3 = vec![vec![1.0], vec![2.0], vec![1.0]];
    let multiplication = helpers::matrix::multiplication::multiply(&matriz2, &matriz3);
    println!("multiplication: {:?}", multiplication);
}
