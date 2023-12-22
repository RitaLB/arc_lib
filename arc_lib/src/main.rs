use std::time::Instant;
mod find_arcos_v2;
//mod find_arcos_v1;
mod ler_pla;
use crate::ler_pla::ler_pla::TabelaVerdade;
use crate::ler_pla::ler_pla::processar_pla;
use crate::find_arcos_v2::find_arcos_v2::find_arcos_v2;
//use crate::find_arcos_v1::find_arcos_v1::find_arcos_v1;

fn main() {

    let filename = "src/pla7.txt";
    let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
    let saidas = minha_tabela.saidas();
    let entradas = minha_tabela.entradas();
    let n_entradas = minha_tabela.n_inputs();

    // Encontrando transições:
    //início:
    // Captura o tempo de início
    let inicio = Instant::now();

    let transicoes = find_arcos_v2(&saidas, n_entradas);
    //find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>) -> Vec<(u8, u8)> 
    //let transicoes = find_arcos_v1(&saidas, &entradas); 
    
    // Captura o tempo de término
    let fim = Instant::now();
    // Calcula a diferença de tempo
    let duracao = fim - inicio;
    //fim

    //impressão tabela verdade:
    println!("Tabela verdade:");
    minha_tabela.imprimir_tabela();

    println!("Transições encontradas:");
     
    // print para v2:
    // Imprime as transições
    println!("{}", transicoes.len());
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

    // print para v1:
    /*
    for tupla in &transicoes {
        println!("({}, {})", tupla.0, tupla.1);
    }
    */
    // impressão tempo processamento:
    // Imprime a duração em milissegundos
    println!("Tempo de processamento: {} ns", duracao.as_nanos());
}
