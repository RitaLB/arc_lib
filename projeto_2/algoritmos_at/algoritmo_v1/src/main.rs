mod find_arcos_v1;
use find_arcos_v1::find_arcos_v1 as fa1;

fn main() {
    let saida: Vec<u8>  =  vec![ 1, 
    1,
    1,
    0] ;

    let entradas: Vec<Vec<u8>> = vec![vec![0, 0, 1, 1], vec![0, 1, 0, 1]];

    //let mut arc: Vec<(u8, u8)> = Vec::new();
    let arc = fa1::find_arcos_v1(&saida,  &entradas);

    // Imprimir 'arc' após as remoções
    for tupla in &arc {
        println!("{:?}", tupla);
        }
}
