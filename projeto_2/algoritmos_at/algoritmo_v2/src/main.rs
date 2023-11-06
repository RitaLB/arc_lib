mod find_arcos;
fn main() {
    let saida: Vec<u8>  =  vec![ 1, 
    1,
    1,
    0] ;

    let entradas: Vec<Vec<u8>> = vec![vec![0, 0, 1, 1], vec![0, 1, 0, 1]];

    arc = find_arcos(saida, 2, entradas);

    // Imprimir 'arc' após as remoções
    for tupla in &arc {
        println!("{:?}", tupla);
        }
}

/*fn main() {
    let saida: Vec<u8>  =  vec![ 1, 
    1,
    1,
    0] ;

    let entradas: Vec<Vec<u8>> = vec![vec![0, 0, 1, 1], vec![0, 1, 0, 1]];

    // criar vetor de tuplas para arcos

    let mut arc: Vec<(u8, u8)> = Vec::new();


    for k in 0..(saida.len() - 1) {
        let mut l = k + 1;
        while l < saida.len() {
            arc.push((k as u8, l as u8));
            l+= 1;
        }
    }

    // selecionar arcos de mudança por arcos de s

    let mut i = 0;

    while i< arc.len() {
        let(l, h) = arc[i];
        if saida[l as usize] == saida[h as usize]{
            arc.remove(i);
        }
        else {
            i += 1;
        }
            
    }

    // selecionando arcos com mudança em apenas 1 entrada
    let mut i = 0;

    while i < arc.len() { // analiza cada arco
        let mut resultado_entradas: Vec<u8> = Vec::new();
        let l = arc[i as usize].0;
        let h = arc[i as usize].1;

        // analisa cada entrada
        let mut j = 0;
        while j < entradas.len() {
            let entrada = &entradas[j];
            resultado_entradas.push(entrada[l as usize] ^ entrada[h as usize]);
            j += 1;
        }
        // resultado do for, por ex: arco (0,30 a resposta vai ser [ 1 1 ] caso as duas entradas A e B mudarem)

        // verificando se há apenas 1 ntrada com mudança
        let mut soma: u8= 0;
        let mut k = 0;

        while k <= resultado_entradas.len(){
            soma += resultado_entradas.pop().unwrap_or(0);
            k +=1;
        }

        if soma != 1 {
            arc.remove(i);
        } else {
            i += 1;
        }
    }

    // Imprimir 'arc' após as remoções
    for tupla in &arc {
        println!("{:?}", tupla);
        }
}*/
