#include <stdio.h>
#include <stdlib.h>

typedef struct Teste {
    int *num;
} Teste;

Teste clone(const Teste *const t);

int main() {
    Teste t = { (int *)malloc(sizeof(int)) };
    *t.num = 100;
    
    Teste t2 = clone(&t);
    free(t2.num);
    
    printf("%d", *t.num);
}

Teste clone(const Teste *const t) {
    Teste t_novo = { (int *)malloc(sizeof(int)) };
    *t_novo.num = *t->num;
    return t_novo;
}