

// biblioteca para lidar com arquivos:
use std::fs::File;

use std::io::prelude::*;

fn main() {
    println!("Aprendendo a ler e escrever um arquivo em Rust!");

    let filename = "/home/ritabarbosa/Documentos/IC/Fase3/rust/projects/codigo1/src/PLA_fulladder.txt";
    println!("{}",&filename);

    let mut f = File::open(filename).expect("file not found");

    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)
        .expect("Erro de leitura do arquivo");

    println!("{}", conteudo);




}
main()
