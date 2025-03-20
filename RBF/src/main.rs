mod helpers;

fn main() {
    let x1 = vec![0, 1, 1, 0];
    let x2 = vec![0, 1, 0, 1];
    let f = vec![0, 2, 1, 1];

    helpers::print_vector::print_v(&x1);
    println!(" ");
    helpers::print_vector::print_v(&x2);
    println!(" ");
    helpers::print_vector::print_v(&f);
}
