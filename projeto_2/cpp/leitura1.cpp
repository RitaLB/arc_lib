#include <iostream>
#include <fstream>
#include <string>


int main() {

    std::cout <<  "Aprendendo a ler um arquivo em C++!!!"

    std::string nomedoarquivo = "codigo1/PLA_fulladder.txt"

    // Criar um objeto ifstream para abrir o arquivo em modo de leitura
    std::ifstream file(filename)

    // Verificar se o arquivo foi aberto com sucesso
    if (!file.is_open()) {
        std::cerr << "Erro ao abrir o arquivo." << std::endl;
        return 1;
    }

}