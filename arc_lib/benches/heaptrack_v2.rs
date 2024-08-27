use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

use{
    std::env,
    arc_lib::*,
    crate::ler_pla_antigo::ler_pla_antigo::TabelaVerdadeAntiga,
    crate::ler_pla_antigo::ler_pla_antigo::processar_pla_antigo,
    crate::find_arcos_v2_antigo::find_arcos_v2::find_arcos_v2,
};

fn main() {
    // Coletando dados da linha de comando
    let filename = env::var("PLA_FILE").unwrap_or_else(|_| "default.pla".to_string());
    let file_path = format!("src/pla_examples/{}", filename);

    // Processar o arquivo PLA
    let minha_tabela: TabelaVerdadeAntiga = processar_pla_antigo(&file_path.to_string());
    let saidas = minha_tabela.saidas();
    let n_entradas = minha_tabela.n_inputs();

    // Executar a função find_arcos_v1 e medir o uso de memória
    let result =  find_arcos_v2( &saidas, n_entradas, 0);

    // Para garantir que o resultado seja utilizado e não otimizado pelo compilador
    println!("{:?}", result);
}