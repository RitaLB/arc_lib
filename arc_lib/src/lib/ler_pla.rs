pub mod ler_pla {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::io::BufReader;

    // structure para armazenar tabela verdade
    pub struct TabelaVerdade {
        n_outputs: usize,
        n_inputs: usize,
        n_t_produtos: usize,
        saidas: Vec<Vec<u8>>,
        entradas: Vec<Vec<u8>>,
    }
    
    impl TabelaVerdade {
        // Método para criar uma nova instância da tabela verdade com valores padrão
        pub fn new() -> Self {
            TabelaVerdade {
                n_outputs: 0,
                n_inputs: 0,
                n_t_produtos: 0,    // Encontrando transições:
                saidas: Vec::new(),
                entradas: Vec::new(),
            }
        }

        pub fn saidas(&self) -> &Vec<Vec<u8>>{
            &self.saidas
        }

        pub fn entradas(&self) -> &Vec<Vec<u8>>{
            &self.entradas
        }

        pub fn n_outputs(&self) -> usize {
            self.n_outputs
        }

        pub fn n_inputs(&self) -> usize {
            self.n_inputs
        }

        pub fn n_t_produtos(&self) -> usize {
            self.n_t_produtos
        }

        // Método para imprimir a tabela verdade
        pub fn imprimir_tabela(&self) {
            // Imprimir cabeçalho
            println!("self.entradas[0].len() = {}", self.entradas[0].len() );
            println!("self.entradas.len() = {}", self.entradas.len() );
            print!(" ");
            for i in 0..self.n_inputs {
                print!("I{}  ", i);
            }
            for i in 0..self.n_outputs {
                print!("O{}  ", i);
            }
            println!();

            // Imprimir separador
            print!("+-");
            for _ in 0..self.n_inputs {
                print!("--+-");
            }
            for _ in 0..self.n_outputs {
                print!("--+-");
            }
            println!();

            // Imprimir linhas da tabela
            for i in 0..self.entradas[0].len() {
                print!(" ");
                for j in 0..self.n_inputs {
                    print!("{}   ", self.entradas[j][i]);
                }
                for j in 0..self.n_outputs {
                    print!("{}   ", self.saidas[i][j]);
                }
                println!();
            }
        }
    }


    pub fn processar_pla(filename: &String) -> TabelaVerdade{
        // instancia de tabela verdade para salvar os dados
        let mut tabela_verdade: TabelaVerdade;
        let linhas_tabela: Vec<String>;

        let dados_pla = match ler_arquivo(filename) {
            Ok(arquivo_pla) => processar_linhas(arquivo_pla),
            Err(err) => {
                eprintln!("Erro ao ler o arquivo: {}", err);
                return TabelaVerdade::new();
            }
        };

        match dados_pla {
            Ok(dados_pla) => {
                tabela_verdade = dados_pla.0;
                linhas_tabela = dados_pla.1;
                /* 
                println!("tabela desordenada:");
                for linha in &linhas_tabela{
                    println!("{}", linha)
                }
                */
            }
            Err(err) => {
                eprintln!("Erro ao processar pla: {}. Verifique se o PLA está conforme padrões", err);
                return TabelaVerdade::new(); // Retorna uma tabela vazia em caso de erro
            }
        }

        match ordenar_pla(&tabela_verdade, linhas_tabela) {
            Ok(pla_ordenado) => {
                /*
                println!("tabela ordenada:");
                for linha in &pla_ordenado{
                    println!("{}", linha)
                };
                */
                salvar_tabela(pla_ordenado, &mut tabela_verdade);

            },
            Err(err) => {
                eprintln!("Erro ao ordenar tabela: {}. Verifique se tabela está correta e completa.", err);
                return TabelaVerdade::new() // Retorna uma tabela vazia em caso de erro
            }
        }

        tabela_verdade
    }

    fn ler_arquivo(filename: &String)->Result<BufReader<File>, Box<dyn std::error::Error>>{
        // Tenta abrir o arquivo em modo de leitura
        let file = File::open(&filename)?;

        // Cria um leitor de buffer para o arquivo
        let reader = io::BufReader::new(file);
        Ok(reader)
    }

    fn processar_linhas(reader: BufReader<File>) -> Result<(TabelaVerdade, Vec<String>), Box<dyn std::error::Error>> {
        let mut minha_tabela: TabelaVerdade = TabelaVerdade::new();
        let mut linhas_tabela: Vec<String> = Vec::new();
    
        for line_result in reader.lines() {
            let linha = line_result?;
            if let Some(primeiro_char) = linha.chars().nth(0) {
                // verifica se é uma informação sobre a tabela, comentário ou dado da tabela
                if primeiro_char == '.' {
                    salvar_dado(&linha, &mut minha_tabela);
                } else if primeiro_char != '#' {
                    //println!("{}", linha);
                    linhas_tabela.push(linha);
                }
            }
        }
    
        Ok((minha_tabela, linhas_tabela))
    }
    

    fn ordenar_pla(minha_tabela: &TabelaVerdade, linhas_tabela: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let numero_entradas = minha_tabela.n_inputs();
        let n_produtos = linhas_tabela.len() ;
        let mut pla_ordenado: Vec<Option<String>> = vec![None;n_produtos];
        println!("len linhas_tabela = {}", linhas_tabela.len());
        println!("pla_ordenado.len() = {}", pla_ordenado.len());
    
        for linha in linhas_tabela {
            println!("{}", linha   );
            // Descobrindo posição na tabela (binário das entradas)
            let string_posicao: String = linha.chars().filter(|c| c.is_digit(10)).take(numero_entradas).collect();
            
            // Transforma string de posição em um número
            if !string_posicao.is_empty() {
                let posicao: usize = usize::from_str_radix(&string_posicao, 2).unwrap();
    
                // Substituído o insert por uma atribuição direta, considerando um vetor com None inicialmente
                pla_ordenado[posicao] = Some(linha);
            }
        }
    
        // Filtra as opções Some, transformando em um vetor de String
        let resultado_final: Vec<String> = pla_ordenado.into_iter().filter_map(|opt| opt).collect();
    
        Ok(resultado_final)
    }
    

    fn salvar_tabela(pla_ordenado: Vec<String> , tabela_verdade: &mut TabelaVerdade){
        for line in pla_ordenado {
            salvar_linha_tabela(line, tabela_verdade);
        }
    }

    fn salvar_dado(linha: &String, tabela_verdade: &mut TabelaVerdade){
        let mut string_dado = String::new();
        let mut string_identificador = String::new();
        for caractere in linha.chars() {

            // Verifica se é uma letra
            if caractere.is_alphabetic(){
                string_identificador.push(caractere);
            }
            // Verifica se o caractere é um número (0-9) na tabela ASCII
            if caractere.is_digit(10) {
                // Concatena o caractere à string de números
                string_dado.push(caractere);
            }
        }

        if string_identificador.len() > 0{
            let mut dado: usize = 0;
            if string_dado.len() > 0{
                dado = string_dado.parse().unwrap();
            }
            match string_identificador.as_str() {
                "p" =>{
                    tabela_verdade.n_t_produtos = dado;
                    println!("p = {}", dado)
                } ,
                "i" => {
                    tabela_verdade.n_inputs = dado;
                    println!("i = {}", dado)
                },
                "o" =>{
                    tabela_verdade.n_outputs = dado;
                    println!("0 = {}", dado)
                } ,
                "e" => println!("fim do arquivo"),
                _ => println!("_ dado = {}", dado),
            }
        }
    }

    fn salvar_linha_tabela(linha: String, tabela_verdade: &mut TabelaVerdade){
        // saidas: Vec<Vec<u8>>, entradas: Vec<Vec<u8>>,
        let mut cont_i : usize = 0;
        let mut entrada: Vec<u8>= vec![];
        let mut saida: Vec<u8> = vec![];
        for caractere in linha.chars() {
            // Verifica se o caractere é um número (0-9) na tabela ASCII ou é indicador "-"
            if caractere.is_digit(10){
                if cont_i < tabela_verdade.n_inputs {
                    if let Some(inner_vector) = tabela_verdade.entradas.get_mut(cont_i) {
                    inner_vector.push(caractere as u8 - b'0');           
                    } else {
                        tabela_verdade.entradas.resize_with(cont_i + 1, Vec::new);
                        tabela_verdade.entradas[cont_i].push(caractere as u8 - b'0');
                    }
                    cont_i = cont_i + 1;
                } else {
                    saida.push(caractere as u8 - b'0');
                }
            } else if caractere == '-'{
                if cont_i <= tabela_verdade.n_inputs {
                    entrada.push(0);
                    cont_i = cont_i + 1;
                } else {
                    saida.push(0);
                }
            }
        }
        tabela_verdade.saidas.push(saida);
    }
}
