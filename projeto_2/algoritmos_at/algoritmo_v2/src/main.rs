mod find_arcos_v2;
use find_arcos_v2::find_arcos_v2 as fa2;

fn main() {
    let saida = vec![1, 1, 1, 0];
    let n_entradas = 2;

    let transicoes = fa2::find_arcos_v2(&saida, n_entradas);

    // Imprime as transições
    for (i, transicao) in transicoes.iter().enumerate() {
        print!("Transição {}: [", i);
        for (j, res) in transicao.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{}", res);
        }
        println!("]");
    }
}