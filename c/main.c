#include <stdio.h>
#define BUFSIZE 1024

int main() {
    int ibuf[BUFSIZE];
    int ii;

    /* ibufは1024バイト分だけ確保されているが、その領域をオーバーして
       1026バイト書き込みをして、メモリを破壊している */
    for (ii = 0; ii < BUFSIZE + 2; ii++) {
        ibuf[ii] = ii;
    }

    return 0;
}