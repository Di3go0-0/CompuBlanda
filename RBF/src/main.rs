use helpers::{clasification::clasification, solver::weight_vector};

mod helpers;

fn main() {
    let x1 = vec![1.0, 4.0, 2.0, 3.0, 2.0];
    let x2 = vec![0.0, 1.0, 0.0, 1.0, 2.0];
    let f = vec![1.0, 5.0, 2.0, 4.0, 4.0];

    let weight_vector = weight_vector(&x1, &x2, &f);
    // println!("vector: {:?}", weight_vector);

    let p = vec![1.0, 1.5];
    println!("pedicction data: {:?}", weight_vector);

    let predicction = clasification(&weight_vector, &p, &x1, &x2);

    println!("predicction: {}", predicction);

    // panic!("Los vectores deben tener la misma longitud y no estar vacíos");
    // let x1 = vec![0.0, 0.0, 1.0, 1.0];
    // let x2 = vec![0.0, 1.0, 0.0, 1.0];
    //
    // let g = helpers::gaussiana::sigma::calculate_sigma(&x1, &x2).unwrap();
    // println!("sigma: {}", g);
}
