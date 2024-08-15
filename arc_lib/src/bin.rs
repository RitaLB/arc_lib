use std::env;
use arc_lib::*;
mod tests;
use crate::testes::*;
use crate ::tests::testes::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let versao_algoritmo = &args[1];
    let filename = &args[2];
    let file_path = format!("src/pla_examples/{}", filename);
    env::set_var("TEST_FILENAME", &file_path);

    if versao_algoritmo =="1"{
        //execute_v1(&file_path)
    } else if versao_algoritmo =="2"{
        execute_v2(&file_path)
    }

}
