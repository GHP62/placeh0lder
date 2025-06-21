#include "../include/fullRemove.h"

void fullRemove(char *filePath)
{
            FILE *fptr = fopen(filePath, "wb+");

            fseek(fptr, 0, SEEK_END);

            size_t len = ftell(fptr);

            srand(time(NULL));

            for(int k = 0; k < 10; k++)
            {
                for(size_t i = 0; i < len; i++)
                {
                    char randnum = rand();
                    fseek(fptr, i, SEEK_SET);
                    fwrite(&randnum, 1, 1, fptr);
                }
            }

            remove(filePath);
}
