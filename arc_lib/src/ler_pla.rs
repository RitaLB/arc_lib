pub mod ler_pla {
    use std::fs::File;
    use std::io::{self, BufRead};

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
                n_t_produtos: 0,
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
            print!("| ");
            for i in 0..self.n_inputs {
                print!("I{} | ", i);
            }
            for i in 0..self.n_outputs {
                print!("O{} | ", i);
            }
            println!();

            // Imprimir separador
            print!("+-");
            for _ in 0..self.n_inputs {
                print!("----+-");
            }
            for _ in 0..self.n_outputs {
                print!("----+-");
            }
            println!();

            // Imprimir linhas da tabela
            for i in 0..self.entradas[0].len() {
                print!("| ");
                for j in 0..self.n_inputs {
                    print!("{}  | ", self.entradas[j][i]);
                }
                for j in 0..self.n_outputs {
                    print!("{}  | ", self.saidas[i][j]);
                }
                println!();
            }
        }
    }


    pub fn processar_pla(filename: &string) -> TabelaVerdade{
        // instancia de tabela verdade para salvar os dados
        let mut tabela_verdade = TabelaVerdade::new();
        let mut linhas_tabela: Vec<&str> = Vec::new();
        let mut pla_ordenado: Vec<&str> = Vec::new();
        
        let dados_pla = match ler_arquivo(filename) {
            Ok(arquivo_pla) => processar_linhas(arquivo_pla),
            Err(err) => {
                eprintln!("Erro ao ler o arquivo: {}", err);
                return TabelaVerdade::new();
            }
        };
        let mut tabela_verdade= TabelaVerdade::new();
        let mut linhas_tabela: vec<&str> = Vec::new();
        let pla_ordenado : Vec<&str> = vec::new();
        match dados_pla {
            Ok(dados_pla) => {
                tabela_verdade = dados_pla.0;
                linhas_tabela = dados_pla.1;
                pla_ordenado  = ordenar_pla(tabela_verdade, linhas_tabela)}
            Err(err) => {
                eprintln!("Erro ao processar pla: {}. Verifique se o PLA está conforme padrões", err);
                TabelaVerdade::new() // Retorna uma tabela vazia em caso de erro
            }
        }

        match pla_ordenado {
            Ok(pla_ordenado) => salvar_tabela(pla_ordenado, &mut tabela_verdade),
            Err(err) => {
                eprintln!("Erro ao ordenar tabela: {}. Verifique se tabela está correta e completa.", err);
                TabelaVerdade::new() // Retorna uma tabela vazia em caso de erro
            }
        }

        Ok(tabela_verdade)
    }

    fn ler_arquivo(filename: &String)->Result<BufReader<File>, Box<dyn std::error::Error>>{
        // Tenta abrir o arquivo em modo de leitura
        let file = File::open(&filename)?;

        // Cria um leitor de buffer para o arquivo
        let reader = io::BufReader::new(file);
        Ok(reader)
    }

    fn processar_linhas(reader : BufReader<File>) -> Result< (TabelaVerdade,Vec<&str>) , Box<dyn std::error::Error>> {
        let dados_pla: (TabelaVerdade,Vec<&str>);
        let minha_tabela: TabelaVerdade = TabelaVerdade::new();
        let linhas_tabela : Vec<&str> = Vec::new();
        for line_result in reader.lines() {
            let linha = line_result?;
            if let Some(primeiro_char) = linha.chars().nth(0) {
                // verifica se é uma informação sobre a tabela, comentário ou dado da tabela
                if primeiro_char =='.'{
                    salvar_dado(linha, &mut minha_tabela);
                } else if primeiro_char != '#' {
                    //println!("{}", linha);
                    linhas_tabela.push(linha);
                }
            }
        Ok(())
    }
    fn ordenar_pla(minha_tabela : TabelaVerdade , linhas_tabela: Vec<&str>) -> Vec<&str>{
        let numero_entradas = minha_tabela.n_inputs();
        let mut pla_ordenado: Vec<&str> = Vec::new();
    
        for linha in linhas_tabela {
            // Descobrindo posição na tabela (binário das entradas)
            let string_posicao: String = linha
                .chars()
                .filter(|c| c.is_digit(10))
                .take(numero_entradas)
                .collect();
    
            // Transforma string de posição em um número
            if !string_posicao.is_empty() {
                let posicao: usize = usize::from_str_radix(&string_posicao, 2).unwrap();
    
                // Insere a linha na posição correspondente no vetor
                pla_ordenado.insert(posicao, linha);
            }
        }
    
        pla_ordenado
    }

    fn salvar_tabela(pla_ordenado: Vec<&str> , tabela_verdade: &mut TabelaVerdade){
        let mut i = 0;
        while i< (pla_ordenado.len()){
            let line = pla_ordenado[i];
            salvar_linha_tabela(line, &mut tabela_verdade);
        }
    }

    ////######################################## anterior:
    // Função principal que lê o arquivo e processa as linhas
    pub fn processar_pla(filename: &String) -> TabelaVerdade {
        match ler_arquivo(filename) {
            Ok(tabela_verdade) => tabela_verdade,
            Err(err) => {
                eprintln!("Erro ao processar o arquivo: {}", err);
                TabelaVerdade::new() // Retorna uma tabela vazia em caso de erro
            }
        }
    }
    fn ler_arquivo(filename: &String)-> Result<TabelaVerdade, Box<dyn std::error::Error>>{
        
        // instancia de tabela verdade para salvar os dados
        let mut tabela_verdade= TabelaVerdade::new();
        
        // Tenta abrir o arquivo em modo de leitura
        let file = File::open(&filename)?;
    
        // Cria um leitor de buffer para o arquivo
        let reader = io::BufReader::new(file);
    
        // Itera pelas linhas do arquivo e processa cada uma delas
        for line_result in reader.lines() {
            let line = line_result?;
            processar_linha(&line, &mut tabela_verdade);
        }
        println!("n_inputs = {}", tabela_verdade.n_inputs());
        println!("n_outputs = {}", tabela_verdade.n_outputs());
        Ok(tabela_verdade)
    }


    fn processar_linha(linha: &String, tabela_verdade: &mut TabelaVerdade) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(primeiro_char) = linha.chars().nth(0) {
            // verifica se é uma informação sobre a tabela, comentário ou dado da tabela
            if primeiro_char =='.'{
                salvar_dado(linha, tabela_verdade);
            } else if primeiro_char != '#' {
                //println!("{}", linha);
                salvar_linha_tabela(linha, tabela_verdade);
            }
        }
        Ok(())
    }

    fn salvar_linha_tabela(linha: &String, tabela_verdade: &mut TabelaVerdade){
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
                "p" => tabela_verdade.n_t_produtos = dado,
                "i" => tabela_verdade.n_inputs = dado,
                "o" => tabela_verdade.n_outputs = dado,
                "e" => println!("fim do arquivo"),
                _ => println!("linha com erro"),
            }
        }

    } 
} 