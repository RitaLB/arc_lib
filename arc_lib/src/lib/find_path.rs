#![allow(non_snake_case)]
pub mod find_path{
    use std::vec;
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub struct Sistem {
        num_att: u32,
        ciclos: u32,
        meus_arcos: vec::Vec<HashMap<i32,i32>>, 
    }

    pub enum HistoryValue {
        Str(&'static str),
        Int(i32),
    }
// implementar retorno do histórico da busca com hashmap de enum 
    pub fn find_path(arcs: &mut Vec<HashMap<i32,i32>>)-> (Vec<i32>, i32){
        let mut OF: Vec<i32> = build_OF(arcs); 
        //println!("{:?}", OF);
        // OF: Vetor no qual cada posição representa um nodo e o valor representa o numero de conexões com saída em falso que ele tem.
        // numero de conexões vom 0 em false ( 0 ou 2) que determinado nodo tem (determinado pela posição). -1 representa que não há arcos para aquele nodo
        //let mut C: Vec<bool> = vec![false; arcs.len()]; // Vetor que armazena se o nodo já foi completdo ou não (Saida e entrada marcados)
        let mut path: Vec<i32> = Vec::new(); // Vetor que armazena o caminho que será percorrido

        let mut ciclos = 0;
        // Enquanto houver arcos a serem percorridos
        for node in  0 ..OF.len(){
            if OF[node] > 0{
                ciclos += 1;
                internal_find_path(arcs, &mut path ,node as i32, &mut OF);
            }
        }
        return (path, ciclos);
    }


    fn internal_find_path(arcs: &mut Vec<HashMap<i32,i32>>, path: &mut Vec<i32>, beginin: i32, OF: &mut Vec<i32>) {
        let mut choosen_conection: Option<(i32, i32)> = None;
        path.push(beginin);
    
            let node_arcs = &mut arcs[beginin as usize];// percorre lista de asjascencias de cada nodo
            //println!("{:?}", node_arcs);
            let mut nodes2 = Vec::new();
            let mut found0 = false;
            for (node, history) in node_arcs.iter() {
                //println!("{}", *node);
                //println!("node = {}, history = {}", *node, *history);
                if *history == 0 {
                    //path.push(*node);
                    //node_arcs.insert(*node, 1);// 01 saiu
                    choosen_conection = Some((*node, 1));
                    OF[beginin as usize] -= 1;
                    found0 = true;
                    //println!(" begining = {} , node = {},  history = 0 , OF = {:?}",beginin, *node, *OF);
                    break;
                } else if *history == 2 { // 10 chegou
                    nodes2.push(*node);
                }
            }

            if found0 == false && !nodes2.is_empty(){
            let mut maior = nodes2[0];
            for node in nodes2.iter(){
                if OF[*node as usize] > OF[maior as usize] {
                    maior = *node;
                }
            }
            choosen_conection = Some((maior, 3));
            OF[beginin as usize] -= 1;
            //println!("begining = {}, node = {}, history = 2,  OF = {:?}", beginin, maior, *OF);
        }

        if let Some((destiny, new_history)) = choosen_conection {
            //println!("{} -> {}", beginin, destiny);
            //println!("{:?}", arcs[destiny as usize]);
           // println!{"historico antigo {}  ->  {} = {:?}",beginin,destiny,  arcs[beginin as usize][&destiny]};
            //println!{"historico antigo {}  ->  {} = {:?}",destiny, beginin, arcs[destiny as usize][&beginin]};

            arcs[beginin as usize].insert(destiny, new_history); // 01 saiu

            
            let old_history = arcs[destiny as usize][&beginin];
            if old_history == 0 {
                arcs[destiny as usize].insert(beginin, 2); // 10 chegou
            } else if old_history == 1 {
                arcs[destiny as usize].insert(beginin, 3); // 11 chegou e saiu
            }
            //println!{"historico begining {}  -> destiny {} = {:?}",beginin,destiny,  arcs[beginin as usize][&destiny]};
            //println!{"historico destiny {}  ->  begining {} = {:?}",destiny, beginin, arcs[destiny as usize][&beginin]};
            //arcs[destiny as usize].insert(beginin, 2); // 10 chegou
            internal_find_path(arcs, path, destiny, OF);
        } else {
            //println!("retornou");
            return;
        }
    }

    pub fn build_OF(arcs: &mut Vec<HashMap<i32,i32>> ) -> Vec<i32>{ // De inicio, todas as conexoes tem 0 false. 
        let mut NC: Vec<i32> =  vec![-1; arcs.len()];
        for i in 0..arcs.len(){
            if arcs[i].len() != 0{
                NC[i] = arcs[i].len() as i32;
            }
        }
        //println!("{:?}", NC);
        return NC;
    }
    pub fn build_arcs_v2(init_arcs: Vec<Vec<usize>>) -> Vec<HashMap<i32,i32>> {
        let mut arcs: Vec<HashMap<i32,i32>>= Vec::with_capacity(init_arcs.len());
        for _ in 0..init_arcs.len() {
            arcs.push(HashMap::new());
        }
        for a in 0..init_arcs.len() {
            for &b in &init_arcs[a] {
                arcs[a].insert(b as i32, 0);
                arcs[b as usize].insert(a as i32, 0);
            }
        }
        //println!("{:?}", arcs);
        arcs
    }


}

