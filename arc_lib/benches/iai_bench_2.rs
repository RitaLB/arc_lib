extern crate arc_lib as bench_lib;
use {
    std::env,
    bench_lib::*,
    //arc_lib::*,
    criterion::{black_box, criterion_group, Criterion},
    crate::ler_pla_antigo::ler_pla_antigo::TabelaVerdadeAntiga,
    crate::ler_pla_antigo::ler_pla_antigo::processar_pla_antigo,
    crate::find_arcos_v2_antigo::find_arcos_v2::find_arcos_v2,
    crate::find_arcos_v1::find_arcos_v1::find_arcos_v1,
};
fn benchmark(c: &mut Criterion) {
    // Coletando dados da linha de comando
    let filename = env::var("PLA_FILE").unwrap_or_else(|_| "default.pla".to_string());
    let file_path = format!("src/pla_examples/{}", filename);

    // Processar o arquivo PLA
    let minha_tabela: TabelaVerdadeAntiga = processar_pla_antigo(&file_path.to_string());
    let saidas = minha_tabela.saidas();
    let entradas = minha_tabela.entradas();
    let n_entradas = minha_tabela.n_inputs();

    // Utilize with_setup para configurar os dados antes do benchmark
    // Benchmark para find_arcos_v1
    c.bench_function("algoritmo_v1", |b| {
            // Uso de black_box para garantir que os resultados não sejam otimizados
        black_box({
            find_arcos_v1(&saidas, &entradas, 0)
        });
    });

    // Benchmark para find_arcos_v1
    c.bench_function("algoritmo_v2", |b| {
        black_box({
            find_arcos_v2( &saidas, n_entradas, 0)
        });
    });
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = benchmark
}
iai::main!(benches);