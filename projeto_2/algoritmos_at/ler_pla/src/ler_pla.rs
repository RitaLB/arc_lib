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
        fn new() -> Self {
            TabelaVerdade {
                n_outputs: 0,
                n_inputs: 0,
                n_t_produtos: 0,
                saidas: Vec::new(),
                entradas: Vec::new(),
            }
        }
    }

    pub fn ler_pla(filename: String)-> Result<TabelaVerdade, Box<dyn std::error::Error>>{
    
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
                println!("{}", linha);
            }
        }
        Ok(())
    }
    
    fn salvar_dado(linha: &String, tabela_verdade: &mut TabelaVerdade)-> Result<(), Box<dyn std::error::Error>>{
        let mut string_dado = String::new();
        for caractere in linha.chars() {
            // Verifica se o caractere é um número (0-9) na tabela ASCII
            if caractere.is_digit(10) {
                // Concatena o caractere à string de números
                string_dado.push(caractere);
            }
        }
        if let Some(identificador) = linha.chars().nth(1) {
            let dado: usize = string_dado.parse()?;
            
            match identificador {
                'p' => tabela_verdade.n_t_produtos = dado,
                'i' => tabela_verdade.n_inputs = dado,
                'o' => tabela_verdade.n_outputs = dado,
                'e' => println!("fim do arquivo"),
                _ => println!("linha com erro"),
            }
    
            Ok(())
        } else {
            Err("Identificador não encontrado".into())
        }
    }
    
}