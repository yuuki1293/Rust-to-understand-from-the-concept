#include <stdio.h>
#include <math.h>

struct Point
{
    double x;
    double y;
};

typedef struct Point Point;

double distance(Point a, Point b)
{
    return sqrt(pow((a.x - b.x), 2) + pow((a.y - b.y), 2));
}

#ifdef TEST
int main()
{
    Point x = {1.0, 2.0};
    Point y = {4.0, 6.0};

    printf("%f\n", distance(x, y));
}
#endif