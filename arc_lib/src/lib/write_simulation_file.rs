pub mod write_simulation_file{
    //use arc_lib::*;
    use std::fs::File;
    use std::io::{self, Write};
    

    use std::io::{BufWriter};
    use std::collections::HashMap;

    // TEST_FILENAME=src/pla_examples/10_c.txt cargo test -- --nocapture

    pub fn write_simulation_path_file(
        simulation_path: &Vec<Vec<(f64, f64)>>,
        filename: &str
    ) -> io::Result<()> {
        // Criar ou abrir o arquivo .cir
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
    //pub fn write_simulation_path(n_inputs: usize, path: &Vec<i32>, ciclos: i32, arc_types: HashMap<(i32, i32), (&str, usize)>, Vx: f64, periodo:f64,slape: f64 ) -> std::io::Result<()>{

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

    // old fn
    fn write_simulation_path_in_blocks(simulation_path: &Vec<Vec<(f64, f64)>>, periodo:f64, duracao_bloco: f64) -> io::Result<()> {

        let mut declaracoes_totais = 0;
        for caminho in simulation_path.iter(){
            declaracoes_totais += caminho.len();
        }

        let mut  iteradores_entradas = Vec::new();
        for _ in 0..simulation_path.len(){
            iteradores_entradas.push(0);
        }

        let mut block_num = 0.0;
        let mut i = 0;
        //maior_caminho -1
        while i < (declaracoes_totais -1){
            println!("Block: {}", block_num);
            let filename = "block".to_string() + &block_num.to_string() + ".cir";
            let mut file = File::create(filename)?;
            
            // Iterar sobre as fontes e associar o nome das fontes a letras do alfabeto
            for (indice, fonte) in simulation_path.iter().enumerate(){
                // Gerar o nome da fonte. A primeira fonte será "Va", a segunda "Vb", e assim por diante.
                let entrie_name = (b'a' + indice as u8) as char;
                let nome_fonte = format!("V{}", entrie_name);

                // Início da declaração, com o nome da fonte
                write!(file, "{} {} gnd PWL (", nome_fonte, entrie_name)?;

                // Iterar sobre as tuplas (tempo, valor) e formatar para PWL
                
                let  (mut time, mut value) = fonte[iteradores_entradas[indice]];
                let mut tempo_limite = time - ((block_num) * duracao_bloco );
                //println!("TEmpo limite: fora do while {}", tempo_limite);
                //let mut tempo_escrito = (time%((block_num) as f64 ));
                let mut tempo_escrito = time - ((block_num * duracao_bloco ) as f64);
                let mut tempo_escrito_formatado = format!("{:.2}", tempo_escrito);  

                while (tempo_limite < duracao_bloco) & (iteradores_entradas[indice] < fonte.len()){
                    println!("entrou no while");
                    println!("tempo_limite: entrou no while {}", tempo_limite);
                    write!(file, "{}n {} ", tempo_escrito_formatado, value)?;
                    i += 1;
                    println!("i: {}", i);
                    
                    (time, value) = fonte[iteradores_entradas[indice]];
                    //tempo_escrito = (time%((block_num) as f64 ));
                    println!("time = {}", time);
                    println!("block_num = {}", block_num);
                    println!("duracao_bloco = {}", duracao_bloco);
                    println!("block_num * duracao_bloco = {}", block_num * duracao_bloco);
                    println!("block_num * duracao_bloco as f64 = {}", (block_num * duracao_bloco )as f64);
                    tempo_escrito = time - ((block_num * duracao_bloco )as f64);
                    tempo_escrito_formatado = format!("{:.2}", tempo_escrito);
                    println!("tempo_escrito: {}", tempo_escrito);
                    println!("tempo_escrito_formatado: {}", tempo_escrito_formatado);
                    tempo_limite = time  - ((block_num ) * duracao_bloco );
                    iteradores_entradas[indice] += 1;
                    println!("TEmpo limite: {}", tempo_limite);
                    println!("declaracoes_totais: {}", declaracoes_totais);

                } 

                // Remover o último espaço e fechar a declaração
                write!(file, ")\n")?;
            }
            block_num += 1.0;
        }
            
            Ok(())
    }
}