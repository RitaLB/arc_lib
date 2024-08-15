pub mod testes{
    //use arc_lib::*;

    use crate::ler_pla::ler_pla::TabelaVerdade;
    use crate::ler_pla::ler_pla::processar_pla;
    use crate::find_arcos_v2::find_arcos_v2::find_arcos_v2;
    //use crate::find_arcos_v1::find_arcos_v1::find_arcos_v1;
    use crate::find_path::find_path::*;

    /*
    pub fn execute_v1(filename: &str){

        let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
        let saidas = minha_tabela.saidas();
        let entradas = minha_tabela.entradas();

        //let inicio = Instant::now();

        //find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>) -> Vec<(u8, u8)> 
        let transicoes = find_arcos_v1(&saidas, &entradas, 0); 

        //let duracao = Instant::now()- inicio;

        //println!("Tabela verdade:");
        //minha_tabela.imprimir_tabela();

        println!("Transições encontradas:");
        println!("Number Of Transitions found: {}", transicoes.len());
        
        for tupla in &transicoes {
            println!("({}, {})", tupla.0, tupla.1);
        }
        
        //println!("Tempo de processamento: {} ns", duracao.as_nanos());

    }
    */

    pub fn execute_v2(filename: &str){
        let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
        println!("{}", filename);
        let saidas = minha_tabela.saidas();
        let n_entradas = minha_tabela.n_inputs();

        //let inicio = Instant::now();

        let transicoes = find_arcos_v2(&saidas, n_entradas, 0); 

        let mut arcos = build_arcs_v2(transicoes);
        for numero in 0..arcos.len(){
            println!("{} -> {:?}", numero, arcos[numero]);
        }
        let (path , ciclos) = find_path(&mut arcos);
        
        println!("Path: {:?}", path);
        println!("Ciclos: {:?}", ciclos);

    }

    
}

// Módulo de testes
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    // Importa todas as funções do módulo pai (necessário para acessar a função 'soma')
    use super::*;
    fn check_used_arcs(init_arcs: &mut Vec<HashMap<i32,i32>>, path: Vec<i32>) -> bool {
        let arcs = HashMap::new();
        for a in 0.. init_arcs.len(){
            for b in init_arcs[a].keys(){
                arcs.insert((a, *b), false);
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

    #[test]
    fn test_find_path() {

        // Obtém o valor da variável de ambiente
        let filename = env::var("TEST_FILENAME").expect("Arquivo de teste não definido");

        // Agora você pode usar o `filename` no seu teste
        let minha_tabela: TabelaVerdade = processar_pla(&filename.to_string());
        let saidas = minha_tabela.saidas();
        let n_entradas = minha_tabela.n_inputs();

        let transicoes = find_arcos_v2(&saidas, n_entradas, 0); 

        let mut arcos = build_arcs_v2(transicoes);
        for numero in 0..arcos.len() {
            println!("{} -> {:?}", numero, arcos[numero]);
        }
        let (path, ciclos) = find_path(&mut arcos);
        assert!(check_used_arcs(&mut arcos, path));
    }
}