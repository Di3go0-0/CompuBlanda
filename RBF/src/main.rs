mod helpers;

fn main() {
    let v1 = vec![0, 0, 1, 1];
    let v2 = vec![0, 1, 0, 1];

    // let distancia = helpers::utils::centers_distance(&v1, &v2);
    // println!("La distancia es: {}", distancia);
    // let e = helpers::utils::e(distancia);
    // println!("e: {}", e);

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
}
