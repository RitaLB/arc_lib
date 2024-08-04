//use std::collections::HashMap;
/*
pub mod find_path{
    use std::collections::HashMap;

    pub struct Sistem {
        num_att: u32,
        ciclos: u32,
        meus_arcos: HashMap<usize, NodeNumber>, 
    }

    pub struct NodeNumber {
        ATT: bool,
        ID: usize,
        FF: u16,
        TT: u16,
        TF: u16,
        conexoes: Vec<Conexao>,
    }

    pub struct Conexao {
        destino: usize,
        historico: u8,
    }

    impl Conexao {
        pub fn new(dest: usize) -> Self {
            Conexao{
                destino: dest,
                historico: 0,
            }
        }
    }

    impl NodeNumber {
        pub fn verify_tt(&mut self) {
            if self.TT == self.conexoes.len() as u16 {
                self.ATT = true;
            }
        }

        pub fn new(id: usize , m_conexoes: Vec<Conexao>) -> Self {
            NodeNumber{
                ATT: false,
                ID: id,
                FF: 0,
                TT: 0,
                TF: 0,
                conexoes: m_conexoes,
            }
        }

        pub fn addConection(&self, conexao: Conexao) {
            self.conexoes.push(conexao);
        }
    }

    impl Sistem {

        pub fn new1(&self, conexoes: Vec<Vec<usize>>) -> Self {
            Sistem {
                num_att: 0,
                ciclos: 0,
                meus_arcos: Self::fill_arcsV1(conexoes), 
            }
        }

        pub fn new2(&self, conexoes: Vec<Vec<usize>>) -> Self {
            Sistem {
                num_att: 0,
                ciclos: 0,
                meus_arcos: Self::fill_arcsV1(conexoes), 
            }
        }


        pub fn fill_arcsV1(conexoes: Vec<(u8, u8)>) -> HashMap<usize, NodeNumber> {
            let mut arcos = HashMap::new();
        
            for (x, y) in conexoes {
                // adicionando x
                let node_number = arcos.entry(x as usize).or_insert(NodeNumber::new(x, vec![]));
                node_number.conexoes.push(Conexao::new(y));
                // adicionando y
                let node_number = arcos.entry(y as usize).or_insert(NodeNumber::new(y, vec![]));
                node_number.conexoes.push(Conexao::new(x));
            }
        
            arcos
        }

        fn fill_arcsV2(conexoes: Vec<Vec<usize>>) -> HashMap<usize, NodeNumber> {
            // Lógica para preencher os arcos
            Vec::new() // Retornando um vetor vazio temporariamente
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn fill_arcsV1_test() {
        let vetor: Vec<(usize, usize)> = vec![
        (1, 2),
        (3, 4),
        (5, 6),
        (7, 8),];

        let my_map:  HashMap<usize, NodeNumber> = fill_arcsV1(vetor);
        // Iterando sobre o HashMap e imprimindo cada chave e valor
        for (key, value) in &my_map {
            println!("Chave: {}", key);
            println!("Valor: {:?}", value);
        }
        //assert_eq!(soma(2, 2), 4);
        //assert_eq!(soma(-1, 1), 0);
        //assert_eq!(soma(0, 0), 0);

    }
}
fn main() {
    let vetor: Vec<(u8, u8)> = vec![
        (1, 2),
        (3, 4),
        (5, 6),
        (7, 8),];

        let my_map:  HashMap<usize, NodeNumber> = fill_arcsV1(vetor);
        // Iterando sobre o HashMap e imprimindo cada chave e valor
        for (key, value) in &my_map {
            println!("Chave: {}", key);
            println!("Valor: {:?}", value);
        }
}
*/