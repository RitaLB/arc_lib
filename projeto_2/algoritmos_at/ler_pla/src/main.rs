mod ler_pla;
use ler_pla::ler_pla as pla;

fn main(){
    // Nome do arquivo que você deseja ler
    let filename = "src/pla1.txt";
    let minha_tabela:pla::TabelaVerdade = pla::processar_pla(&filename.to_string());
    
    print!("\nentradas\n");
    for vetor_interno in minha_tabela.entradas() {
        // Iterando sobre cada vetor interno
        for &elemento in vetor_interno {
            print!("{}", elemento);
        }
        println!();  // Nova linha entre os vetores internos
    }
    print!("\nsaídas\n");
    for vetor_interno in minha_tabela.saidas() {
        // Iterando sobre cada vetor interno
        for &elemento in vetor_interno {
            print!("{}", elemento);
        }
        println!();  // Nova linha entre os vetores internos
    }
}
/* 
fn main() -> io::Result<()> {
    // Nome do arquivo que você deseja ler
    let filename = "pla1.txt";

    // Tenta abrir o arquivo em modo de leitura
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Erro ao abrir o arquivo: {}", err);
            return Err(err);
        }
    };

    // Cria um leitor de buffer para o arquivo
    let reader = io::BufReader::new(file);

    // Itera pelas linhas do arquivo e imprime cada uma delas
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}*/

