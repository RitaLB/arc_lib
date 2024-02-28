
fn main() {
    let vetor: Vec<(u8, u8)> = vec![(1, 2), (3, 4), (5, 6)];

    for (x, y) in &vetor {
        println!("Primeiro item da tupla: {}", x);
        println!("Segundo item da tupla: {}", y);
    }
}
