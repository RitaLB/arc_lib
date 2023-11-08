use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Nome do arquivo que você deseja ler
    let filename = "pla1.txt";

    // Tenta abrir o arquivo em modo de leitura
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Erro ao abrir o arquivo: {}", err);
            return Err(err);
        }
    };

    // Cria um leitor de buffer para o arquivo
    let reader = io::BufReader::new(file);

    // Itera pelas linhas do arquivo e imprime cada uma delas
    for line in reader.lines() {
        processar_linha(line)
    }

    Ok(())
}

fn processar_linha(linha: str ){
    if linha[0] =! '.' && linha[0] =! '#'{
        println!("{}", line?);
    }
}