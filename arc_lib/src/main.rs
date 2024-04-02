use std::time::Instant;
use std::env;
use crate::ler_pla::ler_pla::TabelaVerdade;
use crate::ler_pla::ler_pla::processar_pla;
use crate::find_arcos_v2::find_arcos_v2::find_arcos_v2;
use crate::find_arcos_v1::find_arcos_v1::find_arcos_v1;
mod find_arcos_v2;
mod find_arcos_v1;
mod ler_pla;

fn main() {
    let args: Vec<String> = env::args().collect();
    let versao_algoritmo = &args[1];
    let filename = &args[2];

    if versao_algoritmo =="1"{
        execute_v1(filename)
    } else if versao_algoritmo =="2"{
        execute_v2(filename)
    }

}

fn execute_v1(filename: &str){

    let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
    let saidas = minha_tabela.saidas();
    let entradas = minha_tabela.entradas();

    let inicio = Instant::now();

    //find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>) -> Vec<(u8, u8)> 
    let transicoes = find_arcos_v1(&saidas, &entradas, 0); 

    let duracao = Instant::now()- inicio;

    println!("Tabela verdade:");
    minha_tabela.imprimir_tabela();

    println!("Transições encontradas:");

    for tupla in &transicoes {
        println!("({}, {})", tupla.0, tupla.1);
    }

    println!("Tempo de processamento: {} ns", duracao.as_nanos());

}

fn execute_v2(filename: &str){
    let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
    let saidas = minha_tabela.saidas();
    let n_entradas = minha_tabela.n_inputs();

    let inicio = Instant::now();

    let transicoes = find_arcos_v2(&saidas, n_entradas, 0); 
    
    let fim = Instant::now();

    let duracao = fim - inicio;

    println!("Tabela verdade:");
    minha_tabela.imprimir_tabela();

    println!("Transições encontradas:");

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
    println!("Tempo de processamento: {} ns", duracao.as_nanos());

}