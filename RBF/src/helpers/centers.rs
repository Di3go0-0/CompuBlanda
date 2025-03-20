// Calculo de los centros (x1, x2)

pub fn get_center(x1: &Vec<i32>, x2: &Vec<i32>, i: usize) -> Option<Vec<i32>> {
    if i < x1.len() && i < x2.len() {
        Some(vec![x1[i], x2[i]])
    } else {
        None
    }
}
