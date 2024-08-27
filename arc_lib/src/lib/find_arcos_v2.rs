pub mod find_arcos_v2 {
    use std::collections::HashMap;

    pub fn find_arcos_v2(saidas:  &HashMap<i32, Vec<i32>>, n_entradas: usize, nbit_saida: usize) -> Vec<Vec<usize>> {
        // Número de pinos de entrada do circuito = n_entradas
        // Vetor contendo todas as transições que você deseja
        let mut transicoes: Vec<Vec<usize>> = Vec::new();

        // Itera por todas as entradas possíveis (1 << k é o mesmo que 2 elevado a k)
        // 2^n_entrada =  número de linhas da tabela verdade para n entradas
        for n in 0..(1 << n_entradas) {
            // Insere um vetor vazio, não sei se é necessário, mas vamos incluir
            transicoes.push(Vec::new());

            // O bit aqui representa qual pino de entrada estamos invertendo
            for bit in 0..n_entradas {
                // O res aqui é o vetor de entrada com o bit invertido
                // n é a linha atual da tabela verdade (vetor de entrada atual)
                // A operação 1 << bit cria um número inteiro que em binário tem apenas um 1 na posição [bit]
                // Fazendo XOR de n com este bit, apenas esse bit de n será invertido
                let res = n ^ (1 << bit);

                // Ignoramos vetores menores que o analisado para evitar repetições
                // Inserimos esse vetor como um dos vetores a "1 bit de distância de n"
                if res > n {
                    /*
                    if saidas[n][nbit_saida] != saidas[res][nbit_saida] {
                        transicoes[n].push(res); 
                    }
                    */
                    if let (Some(saida_n), Some(saida_res)) = (saidas.get(&n), saidas.get(&res)) {
                        if saida_n[nbit_saida] != saida_res[nbit_saida] {
                            transicoes[n as usize].push(res as usize);
                        }
                    } 
                }
            }
        }
        transicoes
    }
}