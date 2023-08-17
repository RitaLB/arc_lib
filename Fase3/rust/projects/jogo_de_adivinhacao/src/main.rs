use std::io;
// io --> biblioteca de entrada e saída 
extern crate rand;

use rand::Rng;

fn main() {
    println!("Advinhe o número!");

        // gerando número aleatório:
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o seu palpite.");

     // let mut --> variável mutável
     // A sintaxe :: na linha ::new indica que new é uma função associada do tipo String. 
     //Uma função associada é implementada sobre um tipo, neste caso String, em vez de uma instância particular de String. Algumas linguagens dão a isso o nome método estático.
    let mut palpite = String::new();

    // chama o método read_line do handle da entrada padrão para obter entrada do usuário. 
    //Também estamos passando um argumento para read_line: &mut palpite.
    // &mut palpite referência mutável de palpite
    
    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", palpite);

    
}