
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

use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::HashMap;

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
    
    let simulation_path = fount_declaration(n_entradas, &path,  0.7, 2.0, 0.01);
    println!("Path: {:?}", path);
    println!("Ciclos: {:?}", ciclos);
    for vec in simulation_path{
        println!("{:?}", vec);
    }
    //println!("Simulation Path: {:?}", simulation_path);

}

pub fn write_simulation_path(n_inputs: usize, path: &Vec<i32>, ciclos: i32, arc_types: HashMap<(i32, i32), (&str, usize)>, Vx: f64, periodo:f64,slape: f64 ) -> std::io::Result<()>{

    let fontes: Vec<Vec<(f64, f64)>> = fount_declaration(n_inputs, path, Vx, periodo, slape);
    // Create a new file for writing
    let file = File::create("output.txt")?;

    // Create a buffered writer to write to the file
    let mut writer = BufWriter::new(file);

    
    // Write some data to the file
    writer.write_all(b"Hello, world!\n")?;
    writer.write_all(b"Rust is awesome.\n")?;

    // Flush the writer to ensure all data is written to disk
    writer.flush()?;

    println!("Write Operation Successful");
    Ok(())
}

pub fn fount_declaration(n_inputs: usize, path: &Vec<i32>, Vx: f64, periodo:f64,slape: f64) -> Vec<Vec<(f64, f64)>>{
    // Inicializa um vetor de vetores com n entradas, cada vetor sendo vazio inicialmente.
    let mut fontes: Vec<Vec<f64>> = vec![Vec::new(); n_inputs];

    let mut declarated_founts: Vec<Vec<(f64, f64)>> = vec![Vec::new(); n_inputs];

    // criando vetor com valores das entradas em cada tempo
    // Para cada valor no vetor original...
    for (t, &val) in path.iter().enumerate() {
        print !("t: {}, val: {}\n", t, val);
        // Itera sobre as n entradas (bits relevantes) para cada tempo.
        for i in 0..n_inputs {
            // Extrai o valor do i-ésimo bit.
            let bit_value = (val >> i) & 1;
            println!("bit_value: {}", bit_value);
            // Adiciona o valor do bit ao vetor correspondente à i-ésima entrada.
            if bit_value == 1{
                fontes[i].push(Vx);
            } else {
                fontes[i].push(0.0);
            }
        }
    }
    for fonte in &fontes{
        println!("{:?}", fonte);
    }

    // criando vetor com a dupa valor da entrada / tempo
    // padrão de declaração: (0n 0.7 6n 0.7 6.01n 0 8n 0 8.01n 0.7 10n 0.7)
        // ou seja: (t0 valor t0_final valor t1 valor t1_final valor ... tn valor tn_final valor) onde tn+1 = tn_final + slape e tn_final = tn + (periodo * num tempos que não muda valor)
        // importante notar que o tn final corresponde ao valor do derradeiro tempo que a entrada possui mesmo valor antes de mudar de valor

    // computando valor inicial
    let mut valores_atuais = Vec::new();
    let mut tempos_atuais = Vec::new();
    for i in 0..n_inputs{
        // inserindo (0, valor)
        declarated_founts[i].push((0.0, fontes[i][0]));
        valores_atuais.push(fontes[i][0]);
        tempos_atuais.push(0.0);
    }

    for i in 1..path.len(){
        for j in 0..n_inputs{
            tempos_atuais[j] += periodo;
            if fontes[j][i] != valores_atuais[j]{
                // inserindo (tn_final, valor)
                declarated_founts[j].push((tempos_atuais[j], valores_atuais[j]));
                // inserindo (tn, valor)
                declarated_founts[j].push((tempos_atuais[j]+ slape, fontes[j][i]));
                valores_atuais[j] = fontes[j][i];
            }
        }
    }

    declarated_founts
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
