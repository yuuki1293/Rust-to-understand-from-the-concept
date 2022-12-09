#include <stdio.h>
int main(){
    int x = 1;
    int* p = &x;
    int* q = p;

    printf("p-1: %d\n", *p); /* p-1: 1 */

    *q = 2;

    /* qとpは同じメモリ領域を指しているのでpが指すメモリ領域に格納されている値も変化 */
    printf("p-2: %d\n", *p); /* p-2: 2 */

    /* xも変化する */
    printf("x: %d\n", x); /* x: 2 */

    return 0;
}