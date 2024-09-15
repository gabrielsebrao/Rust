#include <iostream>

class Teste {
public:
    int *num;
    
    Teste(int *num) {
        this->num = num;
    }
    
    Teste(const Teste &t) {
        this->num = new int(*t.num);
    }
    
    ~Teste() {
        delete num;
    }
};

void printarNumero(const Teste &t);

int main() {
    Teste t(new int(100));
    printarNumero(t);
    std::cout << *t.num << '\n';
}

void printarNumero(const Teste &t) {
    std::cout << *t.num << '\n';
}