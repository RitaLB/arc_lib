
use core::{hash, num, time};
use std::env;
//use arc_lib::*;
use std::fs::File;
use std::io::{self, Write};
   
use arc_lib::ler_pla::ler_pla::TabelaVerdade;
use arc_lib::ler_pla_antigo::ler_pla_antigo::TabelaVerdadeAntiga;
use arc_lib::ler_pla::ler_pla::processar_pla;
use arc_lib::ler_pla_antigo::ler_pla_antigo::processar_pla_antigo;
use arc_lib::find_arcos_v1::find_arcos_v1::find_arcos_v1;
use arc_lib::find_arcos_v2::find_arcos_v2::find_arcos_v2;
use arc_lib::find_path::find_path::*;

use std::io::{BufWriter};
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

      // Escrever no arquivo .cir
    if let Err(e) = write_simulation_path_file(&simulation_path) {
        eprintln!("Erro ao escrever no arquivo: {}", e);
    } else {
        println!("Arquivo 'output.cir' gerado com sucesso!");
    }

    if let Err(e) = write_simulation_path_in_blocks(&simulation_path, 2.0, 10) {
        eprintln!("Erro ao escrever no arquivo: {}", e);
    } else {
        println!("Arquivo 'output.cir' gerado com sucesso!");
    }

    println!("Path: {:?}", path);
    println!("Ciclos: {:?}", ciclos);
    for vec in simulation_path{
        println!("{:?}", vec);
    }
    //println!("Simulation Path: {:?}", simulation_path);

}


fn write_simulation_path_in_blocks(simulation_path: &Vec<Vec<(f64, f64)>>, periodo:f64, duracao_bloco: i32) -> io::Result<()> {

    let final_values = get_final_values_blocks(simulation_path, duracao_bloco);
    let mut inicio_bloco= 0.0;
    let mut pos_valor_atual_entrada = vec![0; simulation_path.len() as usize];
    let mut valor_inicio_cada_bloco : HashMap<char, Vec<(usize, f64)>> = HashMap::new();
    
    for block_num in 0 .. final_values.len(){
        let filename = "block".to_string() + &block_num.to_string() + ".cir";
        let mut file = File::create(filename)?;
        
        for (indice, entrada) in simulation_path.iter().enumerate(){
            if entrada.len() > pos_valor_atual_entrada[indice] as usize{
                inicio_bloco = entrada[pos_valor_atual_entrada[indice] as usize].0;
            }
            //inicio_bloco = entrada[pos_valor_atual_entrada[indice] as usize].0;

            if (inicio_bloco != 0.0) && (inicio_bloco - inicio_bloco.floor()) == 0.0{ // se for tempo inteiro, pula pro proximo
                println!("inicio_bloco: {}", inicio_bloco);
                pos_valor_atual_entrada[indice] += 1;
                if entrada.len() > pos_valor_atual_entrada[indice] as usize{
                    println!("novo inicio do bloco = {}", entrada[pos_valor_atual_entrada[indice] as usize].0);
                    inicio_bloco = entrada[pos_valor_atual_entrada[indice] as usize].0.floor();
                    println!("novo inicio do bloco = {}", inicio_bloco);
                }
                //inicio_bloco = entrada[pos_valor_atual_entrada[indice] as usize].0.floor();
            }
            else{
                inicio_bloco = inicio_bloco.floor();
            }
            // Gerar o nome da fonte. A primeira fonte será "Va", a segunda "Vb", e assim por diante.
            let entrie_name = (b'a' + indice as u8) as char;
            let nome_fonte = format!("V{}", entrie_name);
            valor_inicio_cada_bloco.entry(entrie_name).or_insert_with(Vec::new).push((block_num,inicio_bloco));
            // Início da declaração, com o nome da fonte
            write!(file, "{} {} gnd PWL (", nome_fonte, entrie_name)?;

            for (tempo, valor) in entrada.iter(){
                if (*tempo > inicio_bloco) && (*tempo <= final_values[block_num as usize]){ // pegando tempos no intervalo do bloco
                    let mut tempo_relativo = *tempo - inicio_bloco;
                    if tempo_relativo < 1.0{
                        tempo_relativo = 0.0;
                    }
                    let tempo_escrito = format!("{:.2}", tempo_relativo);
                    write!(file, "{}n {} ", tempo_escrito, valor)?;
                    pos_valor_atual_entrada[indice] += 1;
                }
                
            }
            //pos_valor_atual_entrada[indice] += 1;
            // Remover o último espaço e fechar a declaração
            write!(file, ")\n")?;
        }
    }
    println!("valor_inicio_cada_bloco = {:?}", valor_inicio_cada_bloco);

    Ok(())
}

fn get_final_values_blocks(simulation_path: &Vec<Vec<(f64, f64)>>, duracao_bloco: i32) -> Vec<f64>{
    let mut ultimo_tempo_bloco = vec![0.0; simulation_path[0].len() as usize];
    //let mut pos_ultimo_bloco: Vec<(i32, i32)> = vec![(0,0); simulation_path[0].len() as usize]; // (entrada, posicao)

    for entrada in simulation_path.iter(){
        //println!("Nova entrada!");
        let mut i = 0;
        let mut pos = 0;
        let mut  delimitador = i * duracao_bloco;
        for (tempo, _valor) in entrada.iter(){
            let tempo_relativo = *tempo - (delimitador as f64);
            
            //println!("i: {}, tempo: {},  tempo_relativo: {}, ultimo_tempo_bloco[i]: {}", i, tempo,  tempo_relativo, ultimo_tempo_bloco[i as usize]);
            if (*tempo > ultimo_tempo_bloco[i as usize]) & (tempo_relativo <= duracao_bloco as f64){
                ultimo_tempo_bloco[i as usize] = entrada[pos as usize].0;
                //ultimo_tempo_bloco[i as usize] = *tempo;
                //println!("mudou valor. Ultimo_tempo_bloco[{}]: {}",i,  ultimo_tempo_bloco[i as usize]);


            }
            else if tempo_relativo > duracao_bloco as f64{
                println!("Ultimo tempo bloco [{}] - ultimo_tempo[i as usize].floor(): {:?}",i,  ultimo_tempo_bloco[i as usize] -ultimo_tempo_bloco[i as usize].floor());
                if (ultimo_tempo_bloco[i as usize] -ultimo_tempo_bloco[i as usize].floor()) > 0.0{ // para na ultima finalização de periodo
                    println!("Ultimo tempo bloco [{}]: {:?}",i,  ultimo_tempo_bloco[i as usize]);
                    ultimo_tempo_bloco[i as usize] = entrada[pos-1 as usize].0;
                    println!("mudou valor. Ultimo_tempo_bloco[{}]: {}",i,  ultimo_tempo_bloco[i as usize]);
                }
                i += 1; // novo bloco
                delimitador = i * duracao_bloco;

            }
            pos += 1;
        }

    }
    println!("Ultimo tempo bloco: {:?}", ultimo_tempo_bloco);
    let final_values: Vec<f64> = ultimo_tempo_bloco.into_iter().filter(|&x| x != 0.0).collect();
    return final_values;

}

fn write_simulation_path_file(simulation_path: &Vec<Vec<(f64, f64)>>) -> io::Result<()> {
    // Criar ou abrir o arquivo .cir
    let filename = "output.cir";
    let mut file = File::create(filename)?;

    // Iterar sobre as fontes e associar o nome das fontes a letras do alfabeto
    for (i, fonte) in simulation_path.iter().enumerate() {
        // Gerar o nome da fonte. A primeira fonte será "Va", a segunda "Vb", e assim por diante.
        let entrie_name = (b'a' + i as u8) as char;
        let nome_fonte = format!("V{}", entrie_name);

        // Início da declaração, com o nome da fonte
        write!(file, "{} {} gnd PWL (", nome_fonte, entrie_name)?;

        // Iterar sobre as tuplas (tempo, valor) e formatar para PWL
        for (tempo, valor) in fonte {
            write!(file, "{}n {} ", tempo, valor)?;
        }

        // Remover o último espaço e fechar a declaração
        write!(file, ")\n")?;
    }

    Ok(())
}

pub fn fount_declaration(n_inputs: usize, path: &Vec<i32>, Vx: f64, periodo:f64,slape: f64) -> Vec<Vec<(f64, f64)>>{
    let mut fontes: Vec<Vec<f64>> = vec![Vec::new(); n_inputs];

    let mut declarated_founts: Vec<Vec<(f64, f64)>> = vec![Vec::new(); n_inputs];

    // criando vetor com valores das entradas em cada tempo
    // Para cada valor no vetor original...
    for (t, &val) in path.iter().enumerate() {
        //print !("t: {}, val: {}\n", t, val);
        // Itera sobre as n entradas (bits relevantes) para cada tempo.
        for i in 0..n_inputs {
            // Extrai o valor do i-ésimo bit.
            let bit_value = (val >> i) & 1;
            //println!("bit_value: {}", bit_value);
            // Adiciona o valor do bit ao vetor correspondente à i-ésima entrada.
            if bit_value == 1{
                fontes[i].push(Vx);
            } else {
                fontes[i].push(0.0);
            }
        }
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

    let mut maior_tempo = 0.0;
    for i in 1..path.len(){
        for j in 0..n_inputs{
            tempos_atuais[j] += periodo;
            maior_tempo = tempos_atuais[j];
            if fontes[j][i] != valores_atuais[j]{
                // inserindo (tn_final, valor)
                declarated_founts[j].push((tempos_atuais[j], valores_atuais[j]));
                // inserindo (tn, valor)
                declarated_founts[j].push((tempos_atuais[j]+ slape, fontes[j][i]));
                valores_atuais[j] = fontes[j][i];
            }
        }
    }
    for i in 0..n_inputs{
        // inserindo (0, valor)
        declarated_founts[i].push((maior_tempo + periodo, valores_atuais[i]));
    }
    println!("Maior tempo: {}", maior_tempo);

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
