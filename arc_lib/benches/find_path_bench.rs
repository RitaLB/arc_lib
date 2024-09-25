use std::alloc::System;
use{
    std::collections::HashMap,
    std::env,
    arc_lib::ler_pla::ler_pla::TabelaVerdade,
    arc_lib::ler_pla::ler_pla::processar_pla,
    arc_lib::find_arcos_v2::find_arcos_v2::find_arcos_v2,
    //use crate::find_arcos_v1::find_arcos_v1::find_arcos_v1;
    arc_lib::find_path::find_path::*,
};

//  PLA_FILE=3_c.txt cargo bench --bench find_path

fn main() {
    print!("Teste de estresse");
    // Obtém o valor da variável de ambiente
    //let filename = env::var("PLA_FILE").unwrap_or_else(|_| "default.pla".to_string());
    let filename = env::var("PLA_FILE").expect("Arquivo de teste não definido");
    let file_path = format!("src/pla_examples/{}", filename);
   
    let minha_tabela: TabelaVerdade = processar_pla(&file_path);
    let saidas = minha_tabela.saidas();
    let n_entradas = minha_tabela.n_inputs();

    let mut menor_path: Vec<i32> = Vec::new();
    let mut menor = 10000000;
    let mut maior_path: Vec<i32>  = Vec::new();
    let mut maior = 0;
    let mut ocorrencias : HashMap<i32, i32> = HashMap::new();
    for _ in 0 .. 1000{
        let (transicoes, arc_types) = find_arcos_v2(&saidas, n_entradas, 0); 
        let mut arcos = build_arcs_v2(transicoes);
        let (path, ciclos) = find_path(&mut arcos);        
        //println!("Path: {:?}", path);
        *ocorrencias.entry(ciclos).or_insert(0) += 1;
        //println!("Ciclos m: {:?}", ciclos);
        if ciclos < menor{
            menor = ciclos;
            menor_path = path;
        }
        else if ciclos > maior{
            maior = ciclos;
            maior_path = path;
        }
    }
    println!("ocorrências: {:?}", ocorrencias);
    println!("MENOR: {}", menor);
    println!("MENOR PATH: {:?}", menor_path);
    println!("MAIOR: {}", maior);
    println!("MAIOR PATH: {:?}", maior_path);
}