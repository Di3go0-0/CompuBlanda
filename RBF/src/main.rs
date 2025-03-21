mod helpers;

fn main() {
    let x1 = vec![0.0, 0.0, 1.0, 1.0];
    let x2 = vec![0.0, 1.0, 0.0, 1.0];
    let f = vec![0.0, 1.0, 1.0, 2.0];

    let weight_vector = helpers::solver::weight_vector(&x1, &x2, &f);
    println!("vector: {:?}", weight_vector);
}
