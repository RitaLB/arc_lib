
use core::{hash, num, time};
use std::{env, vec};
//use arc_lib::*;
use std::fs::File;
use std::io::{self, Write};
use std::fs::OpenOptions;
   
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

    //find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>) -> Vec<(u8, u8)> 
    let transicoes = find_arcos_v1(&saidas, &entradas, 0); 

    println!("Transições encontradas:");
    println!("Number Of Transitions found: {}", transicoes.len());
    
    for tupla in &transicoes {
        println!("({}, {})", tupla.0, tupla.1);
    }

}


pub fn execute_v2(filename: &str){
    let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
    let saidas: &HashMap<i32, Vec<i32>> = minha_tabela.saidas();
    let n_entradas: usize = minha_tabela.n_inputs();

    let simulation_file_name = "simulation_path.cir";

    // Ajeitar pra já retornar os arcos
    let (transicoes, arc_types) = find_arcos_v2(&saidas, n_entradas, 0); 
    
    let mut arcos = build_arcs_v2(transicoes);
    for numero in 0..arcos.len(){
        println!("{} -> {:?}", numero, arcos[numero]);
    }
    let (path , ciclos) = find_path(&mut arcos);
    let rise_fall_outout_control = find_rise_fall_output_control(&arc_types, &path, &saidas, 0);
    let (simulation_path, rise_fall_entries_control) = fount_declaration(n_entradas, &path,  0.7, 2.0, 0.01);

      // Escrever no arquivo .cir
    if let Err(e) = write_simulation_path_file(&path, &simulation_path, &rise_fall_outout_control, &rise_fall_entries_control, n_entradas as i32, simulation_file_name) {
        eprintln!("Erro ao escrever no arquivo: {}", e);
    } else {
        println!("Arquivo 'output.cir' gerado com sucesso!");
    }

    /*
    if let Err(e) = write_simulation_path_in_blocks(&simulation_path, 2.0, 10) {
        eprintln!("Erro ao escrever no arquivo: {}", e);
    } else {
        println!("Arquivo 'output.cir' gerado com sucesso!");
    }
    */

    println!("Path: {:?}", path);
    println!("Ciclos: {:?}", ciclos);
    for vec in simulation_path{
        println!("{:?}", vec);
    }

}



fn write_delay_measurement(
    path: &Vec<i32>,
    filename: &str,
    rise_fall_output_control: &HashMap<(i32, i32), (&str, i32)>,
    rise_fall_entries_control: &HashMap<(i32, i32), (&str, usize, i32)>,
    n_entradas: i32,
) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(filename)?;

    writeln!(file, "\n* Medindo tempo de propagação:\n")?;
    let mut old_value = path[0];
    for i in 1..path.len() {
        let new_value = path[i];
        let arc = (old_value, new_value);
        //println!("arc = {:?}", arc);

        if let Some((tipo_transicao, numero_mudanca_saida))  = rise_fall_output_control.get(&arc){
            if let Some( (_tipo_mudanca_entrada, entrada_index, num_mudanca_entrada))= rise_fall_entries_control.get(&arc){
                
                let entrada_label = (b'a' + *entrada_index as u8) as char;
                let arco_nome = create_arc_name(*entrada_index, old_value, n_entradas);
                println!("arc = {:?}, arco_nome = {}, entrada_label = {}, entrada_index = {}", arc, arco_nome, entrada_label, entrada_index);
        
                writeln!(file, "* Arco {}", arco_nome)?;
                // Define o tipo de medição: tplh ou tphl, com base nas transições de entrada e saída
                let  measure = match (*tipo_transicao) {
                    "LH" => 
                        format!(".measure tran tplh_{} trig v({}) val= '0.5*0.7' rise={} targ v(s) val= '0.5*0.7' rise={}\n", arco_nome, entrada_label,num_mudanca_entrada, numero_mudanca_saida),
                    "HL" => 
                        format!(".measure tran tphl_{} trig v({}) val= '0.5*0.7' fall={} targ v(s) val='0.5*0.7' fall={}\n", arco_nome, entrada_label, num_mudanca_entrada, numero_mudanca_saida),
                    _ => {
                        // Se não for "LH" nem "HL", apenas ignore (ou tome uma ação apropriada)
                        continue; // Ignora esse caso e passa para o próximo loop
                    }
                };
        
                // Escreve as medições tplh e tphl, se definidas
                if !measure.is_empty() {
                    writeln!(file, "{}", measure)?;
                }
                }
            
        }

        old_value = new_value

    }

    Ok(())
}

fn create_arc_name(entrada_index: usize, val_demais_entradas: i32,n_entradas : i32 ) -> String {

    //println!("entrada_index: {}, val_demais_entradas: {}", entrada_index, val_demais_entradas);
    // Itera por cada bit da entrada, de forma a criar o nome do arco
    let mut bits = Vec::new();
    for i in 0 .. n_entradas{
        //println!("i: {}", i);
        let bit = (val_demais_entradas >> i) & 1;
        bits.push(bit);
    }
    let mut arco_nome = String::new();

    for i in (0..bits.len()).rev() {
        let bit = bits[i];
        if i == entrada_index {
            arco_nome.push((b'a' + entrada_index as u8) as char);
        } else {
            arco_nome.push_str(&bit.to_string());
        }
    }
    //println!("arco_nome: {}", arco_nome);
    return arco_nome;
}

fn find_rise_fall_output_control<'a>(arc_types : &'a HashMap<(i32, i32),(&'a str, usize)> , path : &'a Vec<i32>, saidas:&'a HashMap<i32, Vec<i32>> , n_bit_saida : usize) -> HashMap<(i32, i32), (&'a str,  i32)>{
                                    // k = (val_anterior, val), v = (tipo de transição, número da mudança)
    let mut rise_fall_output_control: HashMap<(i32, i32), (&str, i32)> = HashMap::new();
    let mut rise_control: i32 = 0;
    let mut fall_control: i32 = 0;
    let mut val_anterior: i32 = path[0];
    let mut val_anterior_saida: i32 = saidas[&path[0]][n_bit_saida];

    for &val in path.iter() {
        //println!("val: {}, val_anteriori = {}", val, val_anterior);
        if saidas[&val][n_bit_saida] != val_anterior_saida{
            if saidas[&val][n_bit_saida] == 1{
                rise_control += 1;
                if arc_types.contains_key(&(val_anterior, val)){
                    //println!("LH");
                    rise_fall_output_control.insert((val_anterior, val), ("LH", rise_control));
                }

            } else {
                fall_control += 1;
                if arc_types.contains_key(&(val_anterior, val)){
                    //println!("HL");
                    rise_fall_output_control.insert((val_anterior, val), ("HL", fall_control));
                }
            }
        }
        val_anterior_saida = saidas[&val][n_bit_saida];
        val_anterior = val;
    }
    //println!("rise_fall_output_control: {:?}", rise_fall_output_control);
    return rise_fall_output_control;
}

fn write_simulation_path_file(simple_path: &Vec<i32>, simulation_path: &Vec<Vec<(f64, f64)>>,     rise_fall_output_control: &HashMap<(i32, i32), (&str, i32)>,
rise_fall_entries_control: &HashMap<(i32, i32), (&str, usize, i32)>, n_entradas: i32, simulation_file_name: &str) -> io::Result<()> {
    // Criar ou abrir o arquivo .cir
    let filename = simulation_file_name;
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

    // Escrever medições de tempo de propagação
    write_delay_measurement(&simple_path, filename, rise_fall_output_control, rise_fall_entries_control, n_entradas )?;

    Ok(())
}

pub fn fount_declaration(n_inputs: usize, path: &Vec<i32>, Vx: f64, periodo:f64,slape: f64) -> (Vec<Vec<(f64, f64)>>, HashMap<(i32, i32), (&str, usize, i32)>){
    let mut fontes: Vec<Vec<f64>> = vec![Vec::new(); n_inputs];
    let mut declarated_founts: Vec<Vec<(f64, f64)>> = vec![Vec::new(); n_inputs];

    let mut rise_fall_control: HashMap<(i32, i32), (&str, usize, i32)> = HashMap::new();
    let mut rise_control: Vec<i32> = vec![0; n_inputs];
    let mut fall_control: Vec<i32> = vec![0; n_inputs];
    let mut val_anterior: i32 = path[0];
    let mut val_anterior_entrada: Vec<i32> = vec![0; n_inputs];

    // criando vetor com valores das entradas em cada tempo
    // Para cada valor no vetor original...
    for (t, &val) in path.iter().enumerate() {
        //print !("t: {}, val: {}\n", t, val);
        //println!("t: {}, val: {}", t, val);
        // Add valor inicial
        if t == 0{
            for i in 0..n_inputs{
                // Extrai o valor do i-ésimo bit.
                let bit_value = (val >> i) & 1;
                val_anterior_entrada[i] = bit_value;
            }
            //println!("valç inicial entradas = {:?}", val_anterior_entrada);
        }
        // Itera sobre as n entradas (bits relevantes) para cada tempo.
        for i in 0..n_inputs {
            
            // Extrai o valor do i-ésimo bit.
            let bit_value = (val >> i) & 1;
            //println!("bit value: {}", bit_value);
            //println!("bit_value: {}", bit_value);
            // Adiciona o valor do bit ao vetor correspondente à i-ésima entrada.
            if bit_value == 1{
                //println!("entrou");
                //println!("val_anterior_entrada[{}]: {}", i, val_anterior_entrada[i]);
                fontes[i].push(Vx);
                // se o valor anterior era 0, incrementa o rise_control
                if val_anterior_entrada[i] == 0{
                    rise_control[i] += 1;
                    // k = (val_anterior, val), v = (tipo de mudança, entrada que mudou, número da mudança)
                    rise_fall_control.insert((val_anterior, val), ("R", i, rise_control[i]));
                    //println!("entrada: {}, val_anterior: {}, val: {}, rise_control[{}]: {}",i,  val_anterior, val, i, rise_control[i]);
                }
                val_anterior_entrada[i] = 1;
            } else {
                fontes[i].push(0.0);

                // se o valor anterior era 1, incrementa o rise_control
                if val_anterior_entrada[i] == 1{
                    fall_control[i] += 1;
                    // k = (val_anterior, val), v = (tipo de mudança, entrada que mudou, número da mudança)
                    rise_fall_control.insert((val_anterior, val), ("F", i, fall_control[i]));
                    //println!("entrada: {}, val_anterior: {}, val: {}, fall_control[{}]: {}", i, val_anterior, val, i, fall_control[i]);
                }
                val_anterior_entrada[i] = 0;
            }
        }
        val_anterior = val;
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
            if fontes[j][i] != valores_atuais[j]{ // mudou valor
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

    (declarated_founts, rise_fall_control)
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

            let mut bloco_iniciado = false;
            for (tempo, valor) in entrada.iter(){
                if (*tempo > inicio_bloco) && (*tempo <= final_values[block_num as usize]){ // pegando tempos no intervalo do bloco
                    if !(bloco_iniciado){
                        bloco_iniciado = true;
                        //write!(file, "0n {} ", tempo_escrito, valor)?;
                    }
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
