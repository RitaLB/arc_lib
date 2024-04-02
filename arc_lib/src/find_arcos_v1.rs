pub mod find_arcos_v1 {
    
    pub fn find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>, nbit_saida: usize) -> Vec<(u8, u8)> {
        let linhas: usize = saidas.len();
        let mut arc: Vec<(u8, u8)> = criar_arcos(linhas);
        check_mudanca_s(&mut arc,  &saidas, nbit_saida);
        check_entradas(&mut arc, &entradas);
        return arc;
    }   

    // criar vetor de tuplas com todos os arcos possíveis
    fn criar_arcos(linhas: usize) -> Vec<(u8, u8)>{
        let mut arc: Vec<(u8, u8)> = Vec::new();

        for k in 0..(linhas - 1) {
            let mut l = k + 1;
            while l < linhas {
                arc.push((k as u8, l as u8));
                l+= 1;
            }
        }

        return arc;
    }

    // selecionar arcos de mudança por arcos de s
    // colocando o & eu ocnsigo modificar diretamente o objeto parâmetro ?
    fn check_mudanca_s(arc: &mut Vec<(u8, u8)>, saidas:  &Vec<Vec<u8>> , nbit_saida: usize){
        let mut i = 0;
        while i< (arc.len()){
            // while i< (arc.len() *  saidas.len()){
            let(l, h) = arc[i];
            if saidas[l as usize][nbit_saida] == saidas[h as usize][nbit_saida]{
                arc.remove(i);
            } else {
                i = i+1;
            }
        }
    }

    // selecionando arcos com mudança em apenas 1 entrada
    fn check_entradas(arc: &mut Vec<(u8, u8)>,  entradas: &Vec<Vec<u8>>){
        let mut i = 0;

        // analiza cada arco
        while i < arc.len() { 
            let l = arc[i as usize].0;
            let h = arc[i as usize].1;
    
            // analisa cada entrada
            let mut j = 0;
            let mut soma: u8= 0;
            while j < entradas.len() {
                let entrada = &entradas[j];
                soma = soma + (entrada[l as usize] ^ entrada[h as usize]);
                j += 1;
            }
            // verificando se há apenas 1 ntrada com mudança

            if soma != 1 {
                arc.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

