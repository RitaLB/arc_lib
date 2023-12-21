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
        for caractere in linha.chars() {
            // Verifica se o caractere é um número (0-9) na tabela ASCII
            if caractere.is_digit(10) {
                // Concatena o caractere à string de números
                string_dado.push(caractere);
            }
        }


        if let Some(identificador) = linha.chars().nth(1) {
            let mut dado: usize = 0;
            if string_dado.len() > 0{
                dado = string_dado.parse().unwrap();
            }
            match identificador {
                'p' => tabela_verdade.n_t_produtos = dado,
                'i' => tabela_verdade.n_inputs = dado,
                'o' => tabela_verdade.n_outputs = dado,
                'e' => println!("fim do arquivo"),
                _ => println!("linha com erro"),
            }
        }
    } 
} 