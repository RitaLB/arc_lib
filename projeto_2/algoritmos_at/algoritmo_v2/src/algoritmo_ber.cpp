#include <iostream>
#include <stdio.h>
#include <vector>

using namespace std;

// Dado um numero inteiro o converte para um vetor de tensoes em pinos
vector<float> bin2vector(int n, int bits, float vdd){
    // Vetor de valores em pinos
    vector<float> pinos;
    // Itera pelo numero de bits
    for (int i = 0; i < bits; ++i){
        // (n & 1 << i eh a operacao de E logico entre n e 1 bitshiftado)
        // Isso so eh um valor inteiro diferente de 0 caso n seja 1 no bit i
        // a ? b : c; eh um ternario, eh um versao de uma linha de:
        // if (a) return b; else return c;
        // Lembrando que em C e C++ inteiro positivos avaliam como True, 0 avalia como false
        // Assim so um a ? ou if (a) confere se eh diferente de 0
        float v = (n & 1 << i) ? vdd : 0;
        pinos.push_back(v);
    }
    return pinos;
}

int main()
{
    //numero de pinos de entrada do circuito
    int entradas = 2;
    float vdd = 1;
    //Vetor contendo todas as transicoes que tu quer
    vector<vector<int>> transicoes;
    // Itera por todas entradas possiveis (1 << k eh a mesma coisa que 2 elevado na k)
    for (int n = 0; n < (1 << entradas); n++){
        //Insere um vetor vazio, nao sei se precisa mas vai que
        transicoes.push_back({});
        //Bit aqui representa qual pino da entrada estamos flippando
        for (int bit = 0; bit < entradas; ++bit){
            // res aqui eh o vetor de entrada com o bit flippado
            // a operacao 1 << bit cria um inteiro que em binario so tem 1 na posicao [bit]
            // ao fazer XOR de n com este bit apenas este bit de n sera flippado
            int res = (n ^ 1 << bit);
            // ignoramos vetores novos menores que o analisado para evitar repeticoes
            // inserimos esse vetor como um dos vetores a "1 bit de distancia de n"
            if (res > n) transicoes[n].push_back(res);
        }
    }
    
    for (int i {}; i < 8; ++i)  {
        auto pins = bin2vector(i, 3, 1);
        for (auto& p: pins) {
            cout << p;
        }
        cout << endl;
    }
        

    // // Iteramos novamente por todos os valores de entrada, dessa vez vamos soh imprimilos
    // for (int n = 0; n < (1 << entradas); ++n){
    //     // printamos a representacao binario do nosso vetor
    //     // o %.5b quer dizer que o inteiro sera impesso como binario com 5 casas
    //     printf("%.3b : ", n);
    //     // Iteramos por todas as transicoes validas a partir daquele n (ja armazenada)
    //     for (auto e: transicoes[n]){
    //         // Aplicamos a operacao de XOR em cima de 'e' e 'n'
    //         // Convertemos o inteiro para um vetor de floats que representa a tensao no pino
    //         vector<float> pins = bin2vector((e ^ n), entradas, vdd);
    //         // iteramos pelo vetor de valores nos pinos e is imprimimos
    //         for(auto p: pins){
    //             printf("%.1f ", p);
    //         }
    //         cout << "| ";        // iteramos pelo vetor de valores nos pinos e is imprimimos
    //         for(auto p: pins){
    //             printf("%.1f ", p);
    //         }
    //     }
    //     cout << endl;
    // }
    
    return 0;
}