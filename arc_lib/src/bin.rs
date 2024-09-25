
use core::hash;
use std::env;
//use arc_lib::*;
   
use arc_lib::ler_pla::ler_pla::TabelaVerdade;
use arc_lib::ler_pla_antigo::ler_pla_antigo::TabelaVerdadeAntiga;
use arc_lib::ler_pla::ler_pla::processar_pla;
use arc_lib::ler_pla_antigo::ler_pla_antigo::processar_pla_antigo;
use arc_lib::find_arcos_v1::find_arcos_v1::find_arcos_v1;
use arc_lib::find_arcos_v2::find_arcos_v2::find_arcos_v2;
use arc_lib::find_path::find_path::*;

// TEST_FILENAME=src/pla_examples/10_c.txt cargo test -- --nocapture

pub fn execute_v1(filename: &str){

    let minha_tabela: TabelaVerdadeAntiga = processar_pla_antigo(&filename.to_string());
    let saidas = minha_tabela.saidas();
    let entradas = minha_tabela.entradas();

    //let inicio = Instant::now();

    //find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>) -> Vec<(u8, u8)> 
    let transicoes = find_arcos_v1(&saidas, &entradas, 0); 

    //let duracao = Instant::now()- inicio;

    //println!("Tabela verdade:");
    //minha_tabela.imprimir_tabela();

    println!("Transições encontradas:");
    println!("Number Of Transitions found: {}", transicoes.len());
    
    for tupla in &transicoes {
        println!("({}, {})", tupla.0, tupla.1);
    }
    
    //println!("Tempo de processamento: {} ns", duracao.as_nanos());

}


pub fn execute_v2(filename: &str){
    let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
    let saidas = minha_tabela.saidas();
    let n_entradas = minha_tabela.n_inputs();

    //let inicio = Instant::now();

    let (transicoes, arc_types) = find_arcos_v2(&saidas, n_entradas, 0); 

    let mut arcos = build_arcs_v2(transicoes);
    for numero in 0..arcos.len(){
        println!("{} -> {:?}", numero, arcos[numero]);
    }
    let (path , ciclos) = find_path(&mut arcos);
    
    println!("Path: {:?}", path);
    println!("Ciclos: {:?}", ciclos);

}

pub fn write_simulation_path(path: Vec<usize>, ciclos: Vec<usize>, arc_types: hash::HashMap<(i32, i32), &str>){
    let mut path_str = String::new();
    let mut ciclos_str = String::new();
    for node in path{
        path_str.push_str(&node.to_string());
        path_str.push_str(" ");
    }
    for node in ciclos{
        ciclos_str.push_str(&node.to_string());
        ciclos_str.push_str(" ");
    }
    std::fs::write("simulation_path.txt", path_str).expect("Unable to write file");
    std::fs::write("simulation_ciclos.txt", ciclos_str).expect("Unable to write file");
}
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let versao_algoritmo = &args[1];
    let filename = &args[2];
    let file_path = format!("src/pla_examples/{}", filename);
    

    //env::set_var("TEST_FILENAME", &file_path);

    if versao_algoritmo =="1"{
        execute_v1(&file_path)
    } else if versao_algoritmo =="2"{
        execute_v2(&file_path)
    }
    

}
