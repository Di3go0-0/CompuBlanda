mod helpers;

fn main() {
    let v1 = vec![0, 1];
    let v2 = vec![1, 0];
    let distancia = helpers::utils::centers_distance(&v1, &v2);
    println!("La distancia es: {}", distancia);
}
