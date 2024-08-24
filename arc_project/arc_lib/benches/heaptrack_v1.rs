use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

use{
    std::env,
    arc_lib::*,
    crate::ler_pla_antigo::ler_pla::TabelaVerdade,
    crate::ler_pla_antigo::ler_pla::processar_pla,
    crate::find_arcos_v1::find_arcos_v1::find_arcos_v1,
};

fn main() {
    // Coletando dados da linha de comando
    let filename = env::var("PLA_FILE").unwrap_or_else(|_| "default.pla".to_string());
    let file_path = format!("src/pla_examples/{}", filename);

    // Processar o arquivo PLA
    let minha_tabela: TabelaVerdade = processar_pla(&file_path);
    let saidas = minha_tabela.saidas();
    let entradas = minha_tabela.entradas();

    // Executar a função find_arcos_v1 e medir o uso de memória
    let result = find_arcos_v1(&saidas, &entradas, 0);

    // Para garantir que o resultado seja utilizado e não otimizado pelo compilador
    println!("{:?}", result);
}