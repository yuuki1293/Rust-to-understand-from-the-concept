#include <stdio.h>

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
    int number = value + 10;
    data_a->number_a = &number;
    data_b->number_b = &number;
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
    return 0;
}