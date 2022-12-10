#include <stdio.h>

#define BUFSIZE 1024

int main()
{
    FILE *fp;
    char buf[BUFSIZE];
    int size_read;
    int ret_val;

    if ((fp = fopen("test.txt", "r")) != NULL)
    {
        size_read = fread(buf, 1, BUFSIZE, fp);
        fclose(fp);
        printf("read size: %d\n", size_read);
        ret_val = 0;
    }
    else
    {
        fprintf(stderr, "Failed to open the file");
        ret_val = 1;
    }
    return ret_val;
}
