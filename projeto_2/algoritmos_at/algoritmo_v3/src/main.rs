fn arcos_transicao(saida: &Vec<i32>, n_entradas: usize) -> Vec<Vec<usize>> {
    let tamanho = saida.len();

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
            if res > n && (saida[n] != saida[res]) {
                transicoes[n].push(res);
            }
        }
    }
    transicoes
}

fn main() {
    let saida = vec![1, 1, 1, 0];
    let n_entradas = 2;

    let transicoes = arcos_transicao(&saida, n_entradas);

    // Imprime as transições
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
}