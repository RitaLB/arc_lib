
// biblioteca para lidar com arquivos:
use std::fs::File;

use std::io::prelude::*;

pub fn ler(str path) {

    let filename = path;
    println!("{}",&filename);

    let mut f = File::open(filename).expect("file not found");

    let mut conteudo = String::new();
    f.read_to_string(&mut conteudo)
        .expect("Erro de leitura do arquivo");

    println!("{}", conteudo);




}

