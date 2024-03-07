#ifndef SOMMETP1
#define SOMMETP1

#include <iostream>

int sommeR(int n) {
    if (n == 1) {
        return 1;
    } else {
        return n + sommeR(n - 1);
    }
}

#endif