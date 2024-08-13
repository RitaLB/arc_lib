pub mod find_path_v3{
    use std::vec;
    use std::collections::HashMap;

    pub struct Sistem {
        num_att: u32,
        ciclos: u32,
        meus_arcos: vec::Vec<HashMap<i32,i32>>, 
    }

    pub fn find_path(arcs: &mut Vec<HashMap<i32,i32>>)-> Vec<i32>{
        let mut OF: Vec<i32> = build_OF(arcs); 
        //println!("{:?}", OF);
        // OF: Vetor no qual cada posição representa um nodo e o valor representa o numero de conexões com saída em falso que ele tem.
        // numero de conexões vom 0 em false ( 0 ou 2) que determinado nodo tem (determinado pela posição). -1 representa que não há arcos para aquele nodo
        //let mut C: Vec<bool> = vec![false; arcs.len()]; // Vetor que armazena se o nodo já foi completdo ou não (Saida e entrada marcados)
        let mut path: Vec<i32> = Vec::new(); // Vetor que armazena o caminho que será percorrido

        // Enquanto houver arcos a serem percorridos
        for node in  0 ..OF.len(){
            if OF[node] > 0{
                internal_find_path(arcs, &mut path ,node as i32, &mut OF);
            }
        }
        return path;
    }


    fn internal_find_path(arcs: &mut Vec<HashMap<i32,i32>>, path: &mut Vec<i32>, beginin: i32, OF: &mut Vec<i32>) {
        let mut choosen_conection: Option<(i32, i32)> = None;
        path.push(beginin);
    
        {
            let node_arcs = &mut arcs[beginin as usize];// percorre lista de asjascencias de cada nodo
            println!("{:?}", node_arcs);
            for (node, history) in node_arcs.iter() {
                //println!("{}", *node);
                println!("node = {}, history = {}", *node, *history);
                if *history == 0 {
                    //path.push(*node);
                    //node_arcs.insert(*node, 1);// 01 saiu
                    choosen_conection = Some((*node, 1));
                    OF[beginin as usize] -= 1;
                    println!(" begining = {} , node = {},  history = 0 , OF = {:?}",beginin, *node, *OF);
                    break;
                } else if *history == 2 { // 10 chegou
                    let mut maior = *node;
                    for (node2, history2) in node_arcs.iter(){ // 2°.1 -> encontrar aquela que leva a vertice com maior OF
                        if OF[*node2 as usize] > OF[maior as usize] {
                            maior = *node2;
                            //println!("mudou maior ");
                        }
                    }
                    //path.push(maior);
                    //node_arcs.insert(maior, 3); // 11 chegou e saiu
                    choosen_conection = Some((maior, 3));
                    OF[beginin as usize] -= 1;
                    println!("begining = {}, node = {}, history = 2,  OF = {:?}", beginin, maior, *OF);
                    break;
                }
            }
        }
    
        if let Some((destiny, new_history)) = choosen_conection {
            println!("{} -> {}", beginin, destiny);
            //println!("{:?}", arcs[destiny as usize]);
            println!{"historico antigo {}  ->  {} = {:?}",beginin,destiny,  arcs[beginin as usize][&destiny]};
            println!{"historico antigo {}  ->  {} = {:?}",destiny, beginin, arcs[destiny as usize][&beginin]};

            arcs[beginin as usize].insert(destiny, new_history); // 01 saiu

            
            let old_history = arcs[destiny as usize][&beginin];
            if old_history == 0 {
                arcs[destiny as usize].insert(beginin, 2); // 10 chegou
            } else if old_history == 1 {
                arcs[destiny as usize].insert(beginin, 3); // 11 chegou e saiu
            }
            println!{"historico begining {}  -> destiny {} = {:?}",beginin,destiny,  arcs[beginin as usize][&destiny]};
            println!{"historico destiny {}  ->  begining {} = {:?}",destiny, beginin, arcs[destiny as usize][&beginin]};
            //arcs[destiny as usize].insert(beginin, 2); // 10 chegou
            internal_find_path(arcs, path, destiny, OF);
        } else {
            println!("retornou");
            return;
        }
    }

/*
    fn internal_find_path(arcs: &mut Vec<Vec<Conection>> , path: &mut Vec<i32>, beginin: i32, OF: &mut Vec<i32>){
        let mut choosen_conection: Option<&mut Conection>;
        for conection in arcs[beginin as usize].iter_mut(){
            if conection.historic == 0{ // 1° regra 00
                path.push(conection.destiny);
                conection.update_historic(1); // 01 saiu
                arcs[conection.destiny as usize][beginin as usize].update_historic(2); // 10 chegou
                //arcs[conection.destiny as usize].iter().for_each(|x| x.historic = 2); // 10 entrou
                choosen_conection = Some(conection);
                OF[beginin as usize] -= 1;
                break;
            } else if conection.historic == 2{ // 2°-> 2 -> 10
                // 2°.1 -> encontrar aquela que leva a vertice com maior OF
                let mut maior = conection;
                for conn in arcs[beginin as usize].iter_mut(){
                    if OF[conn.destiny as usize] > OF[maior.destiny as usize]{
                            maior = conn;
                        }
                    }
                path.push(maior.destiny);
                maior.update_historic(3); // 11 chegou e saiu
                arcs[maior.destiny as usize][beginin as usize].update_historic(2); // 10 chegou
                choosen_conection = Some(maior);
                OF[beginin as usize] -= 1;
                break
            } else {
                return; // confirma se de fato acaba de rodar a função
            }
        }
        if let Some(choosen_conection) = choosen_conection {
            internal_find_path(arcs, path, choosen_conection.destiny, OF);
        }

    }
*/
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

    /*
    fn build_arcs_v1(init_arcs: Vec<Arc>) -> Vec<Arc>{ // cria os arcos de ida e volta
        let mut arcs: Vec<Arc> = Vec::new();
        for arc in init_arcs.iter(){
            arcs.push(Arc{a: arc.a, b: arc.b, historic: 0});
            arcs.push(Arc{a: arc.b, b: arc.a, historic: 0});
        }
        return arcs;
    }
    */

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