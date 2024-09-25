
// TEST_FILENAME=filename.txt cargo test -- --nocapture
// Módulo de testes
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use arc_lib::ler_pla::ler_pla::TabelaVerdade;
    use arc_lib::ler_pla::ler_pla::processar_pla;
    use arc_lib::find_arcos_v2::find_arcos_v2::find_arcos_v2;
    //use crate::find_arcos_v1::find_arcos_v1::find_arcos_v1;
    use arc_lib::find_path::find_path::*;

    fn check_used_arcs(init_arcs: &mut Vec<HashMap<i32,i32>>, path: Vec<i32>) -> bool {
        let mut arcs = HashMap::new();
        for a in 0.. init_arcs.len(){
            for b in init_arcs[a].keys(){
                arcs.insert((a as i32, *b), false);
            }
        }

        for i in 0..path.len()-1{
            let a = path[i];
            let b = path[i+1];
            if arcs.contains_key(&(a,b)){
                arcs.insert((a,b), true);
            } else if arcs.contains_key(&(b,a)){
                arcs.insert((b,a), true);} 
        }

        for (_, value) in arcs.iter(){
            if *value == false{
                return false;
            }
        }
        return true;
    }

    #[test]
    fn test_find_path() {

        // Obtém o valor da variável de ambiente
        let filename = env::var("TEST_FILENAME").expect("Arquivo de teste não definido");
        let file_path = format!("src/pla_examples/{}", filename);

        let minha_tabela: TabelaVerdade = processar_pla(&file_path);
        let saidas = minha_tabela.saidas();
        let n_entradas = minha_tabela.n_inputs();

        let (transicoes, arc_types) = find_arcos_v2(&saidas, n_entradas, 0); 

        let mut arcos = build_arcs_v2(transicoes);

        let (path, ciclos) = find_path(&mut arcos);
        assert!(check_used_arcs(&mut arcos, path));
    }

    #[test]
    fn massive_test_find_path(){
        print!("Teste de estresse");
        // Obtém o valor da variável de ambiente
        let filename = env::var("TEST_FILENAME").expect("Arquivo de teste não definido");
        let file_path = format!("src/pla_examples/{}", filename);

        let minha_tabela: TabelaVerdade = processar_pla(&file_path);
        let saidas = minha_tabela.saidas();
        let n_entradas = minha_tabela.n_inputs();

        for _ in 0 .. 1000{
            let (transicoes, arc_types) = find_arcos_v2(&saidas, n_entradas, 0); 
            let mut arcos = build_arcs_v2(transicoes);
            let (path, ciclos) = find_path(&mut arcos);        
            assert!(check_used_arcs(&mut arcos, path));
        }
    }
}
