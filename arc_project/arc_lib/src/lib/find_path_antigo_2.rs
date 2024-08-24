pub mod find_path{

    pub struct Sistem {
        num_att: u32,
        ciclos: u32,
        //meus_arcos: HashMap<usize, NodeNumber>, 
    }

    pub struct Arc {
        a: i32,
        b: i32,
        historic: u8, // se eu amentar ao invés de 00 01 10 11 eu posso ter 0000 0001 0010 0011 0100 0101 0110 0111 1000 1001 1010 1011 1100 1101 1110 1111, eu posso criar apenas 1 arco pra refresentar (a,b) e (b,a) e usar o historic para saber qual é qual
    }

    pub fn find_path(arcs : Vec<Arc>){
        let mut NC: Vec<i32> = build_nc(arcs); // numero de conexões vom 0 em false ( 0 ou 2) que determinado nodo tem (determinado pela posição). -1 representa que não há arcos para aquele nodo

    }

    pub fn build_nc(arcs: Vec<Arc>) -> Vec<i32>{ // De inicio, todas as conexoes tem 0 false. 
        let mut NC: Vec<i32> =  vec![-1; arcs.len()];
        for node in arcs.iter(){
            if NC[node.a as usize] == -1{
                NC[node.a as usize] = 0;
            }
            if NC[node.b as usize] == -1{
                NC[node.b as usize] = 0;
            }
            NC[node.a as usize] += 1;
            //NC[node.b as usize] += 1;
        }
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

    fn build_arcs_v2(init_arcs:  Vec<Vec<i32>> ) -> Vec<Arc>{ // cria os arcos de ida e volta
        let mut arcs: Vec<Arc> = Vec::new();
        for a in 0 .. init_arcs.len(){
            for b in init_arcs[a as usize].copy(){ //Ajeitar em find arcs v2 tipo para ser i32
                arcs.push(Arc{a: a as i32, b: b, historic: 0});
                arcs.push(Arc{a: b, b: a as i32, historic: 0});
            }
        }
        return arcs;
    }
}