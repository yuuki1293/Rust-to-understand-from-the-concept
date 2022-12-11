#include <stdio.h>
#include <stdlib.h>

struct DataA
{
    int *number_a;
};

struct DataB
{
    int *number_b;
};

void setdata(struct DataA *data_a, struct DataB *data_b, int value)
{
    int* number_ptr = (int *)malloc(sizeof(int));
    *number_ptr=value+10;
    data_a->number_a = number_ptr;
    data_b->number_b = number_ptr;
}

int main()
{
    struct DataA data_a_1;
    struct DataB data_b_1;
    struct DataA data_a_2;
    struct DataB data_b_2;

    setdata(&data_a_1, &data_b_1, 1);
    setdata(&data_a_2, &data_b_2, 2);
    printf("to be 11, 11: %d, %d\n",
           *data_a_1.number_a,
           *data_b_1.number_b);
    printf("to be 12, 12: %d, %d\n",
           *data_a_2.number_a,
           *data_b_2.number_b);
    free(data_a_1.number_a);
    free(data_a_2.number_a);
    return 0;
}