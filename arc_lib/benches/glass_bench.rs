
use {
    glassbench::*,
    std::env,
    arc_lib::*,
    crate::ler_pla::ler_pla::TabelaVerdade,
    crate::ler_pla::ler_pla::processar_pla,
    crate::find_arcos_v2::find_arcos_v2::find_arcos_v2,
    crate::find_arcos_v1::find_arcos_v1::find_arcos_v1,
};

pub const WARMUP_ITERATIONS: usize = 10;
pub const ESTIMATE_ITERATIONS: u32 = 1000;
pub const MINIMAL_ITERATIONS: u32 = 500;

fn bench_find_arc_alg(bench: &mut Bench) {
    // Coletando dados da linha de comando
    let filename = env::var("PLA_FILE").unwrap_or_else(|_| "default.pla".to_string());
    let file_path = format!("src/pla_examples/{}", filename);

    // Processar o arquivo PLA
    let minha_tabela: TabelaVerdade = processar_pla(&file_path.to_string());
    let saidas = minha_tabela.saidas();
    let entradas = minha_tabela.entradas();
    let n_entradas = minha_tabela.n_inputs();

    bench.task("algotithm_v1", |task| {
        task.iter(|| {
            // Uso de pretend_used para garantir que os resultados não sejam otimizados
            pretend_used({
                find_arcos_v1(&saidas, &entradas, 0)
            });
        });
    });
    bench.task("algotithm_v2", |task| {
        task.iter(|| {
            pretend_used({
                find_arcos_v2( &saidas, n_entradas, 0)
            })
        });
    });
}

glassbench!(
    "Find Arcas Algorithms",
    bench_find_arc_alg,
    // you can pass other task defining functions here

);