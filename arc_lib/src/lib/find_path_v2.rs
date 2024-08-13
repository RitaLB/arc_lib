pub mod find_path_v2{
    use std::vec;

    pub struct Sistem {
        num_att: u32,
        ciclos: u32,
        meus_arcos: vec::Vec<Vec<Conection>>, 
    }

    pub struct Arc {
        a: i32,
        b: i32,
        historic: u8, // se eu amentar ao invés de 00 01 10 11 eu posso ter 0000 0001 0010 0011 0100 0101 0110 0111 1000 1001 1010 1011 1100 1101 1110 1111, eu posso criar apenas 1 arco pra refresentar (a,b) e (b,a) e usar o historic para saber qual é qual
    }

    #[derive(Debug)]
    pub struct Conection{
        destiny: i32,
        historic: u8,
    }

    impl Conection{
        pub fn update_historic(&mut self, new_historic: u8){
            self.historic = new_historic;
        }
        
    }

    pub fn find_path( arcs : &mut Vec<Vec<Conection>>)-> Vec<i32>{
        let mut OF: Vec<i32> = build_OF(arcs); 
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


    fn internal_find_path(arcs: &mut Vec<Vec<Conection>>, path: &mut Vec<i32>, beginin: i32, OF: &mut Vec<i32>) {
        let mut choosen_conection: Option<(i32, i32)> = None;
    
        {
            let node_arcs = &mut arcs[beginin as usize];
    
            for (i, conection) in node_arcs.iter_mut().enumerate() {
                if conection.historic == 0 {
                    path.push(conection.destiny);
                    conection.update_historic(1);
                    choosen_conection = Some((conection.destiny, beginin));
                    OF[beginin as usize] -= 1;
                    break;
                } else if conection.historic == 2 {
                    let mut maior = i;
                    for (j, conn) in node_arcs.iter().enumerate() {
                        if OF[conn.destiny as usize] > OF[node_arcs[maior].destiny as usize] {
                            maior = j;
                        }
                    }
                    path.push(node_arcs[maior].destiny);
                    node_arcs[maior].update_historic(3);
                    choosen_conection = Some((node_arcs[maior].destiny, beginin));
                    OF[beginin as usize] -= 1;
                    break;
                }
            }
        }
    
        if let Some((destiny, beginin)) = choosen_conection {
            //println!("{} -> {}", beginin, destiny);
            //println!("{:?}", arcs[destiny as usize]);
            arcs[destiny as usize][beginin as usize].update_historic(2); // 10 chegou
            internal_find_path(arcs, path, destiny, OF);
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
    pub fn build_OF(arcs: &mut Vec<Vec<Conection>> ) -> Vec<i32>{ // De inicio, todas as conexoes tem 0 false. 
        let mut NC: Vec<i32> =  vec![-1; arcs.len()];
        for i in 0..arcs.len(){
            if arcs[i].len() != 0{
                NC[i] = arcs[i].len() as i32;
            }
        }
        println!("{:?}", NC);
        return NC;
    }

    fn build_arcs_v1(init_arcs: Vec<Arc>) -> Vec<Arc>{ // cria os arcos de ida e volta
        let mut arcs: Vec<Arc> = Vec::new();
        for arc in init_arcs.iter(){
            arcs.push(Arc{a: arc.a, b: arc.b, historic: 0});
            arcs.push(Arc{a: arc.b, b: arc.a, historic: 0});
        }
        return arcs;
    }

    pub fn build_arcs_v2(init_arcs: Vec<Vec<usize>>) -> Vec<Vec<Conection>> {
        let mut arcs: Vec<Vec<Conection>> = Vec::with_capacity(init_arcs.len());
        for _ in 0..init_arcs.len() {
            arcs.push(Vec::new());
        }
        for a in 0..init_arcs.len() {
            for &b in &init_arcs[a] {
                arcs[a ].push(Conection { destiny: b as i32, historic: 0 });
                arcs[b as usize].push(Conection { destiny: a as i32, historic: 0 });
            }
        }
        println!("{:?}", arcs);
        arcs
    }
}