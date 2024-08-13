pub mod testes{
    //use arc_lib::*;

    use crate::ler_pla::ler_pla::TabelaVerdade;
    use crate::ler_pla::ler_pla::processar_pla;
    use crate::find_arcos_v2::find_arcos_v2::find_arcos_v2;
    use crate::find_arcos_v1::find_arcos_v1::find_arcos_v1;
    use crate::find_path_v3::find_path_v3::*;

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
        let saidas = minha_tabela.saidas();
        let n_entradas = minha_tabela.n_inputs();

        //let inicio = Instant::now();

        let transicoes = find_arcos_v2(&saidas, n_entradas, 0); 

        /*
        for (i, transicao) in transicoes.iter().enumerate() {
            print!("Transição {}: [", i);
            for (j, res) in transicao.iter().enumerate() {
                if j > 0 {
                    print!(", ");
                }
                print!("{}", res);
            }
            println!("]");
        }
        */

        let mut arcos = build_arcs_v2(transicoes);
        for numero in 0..arcos.len(){
            println!("{} -> {:?}", numero, arcos[numero]);
        }
        let path = find_path(&mut arcos);
        
        println!("Path: {:?}", path);
        //let fim = Instant::now();

        //let duracao = fim - inicio;

        //println!("Tabela verdade:");
        //minha_tabela.imprimir_tabela();

        //println!("Tempo de processamento: {} ns", duracao.as_nanos());

    }
}