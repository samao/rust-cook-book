#include <stdio.h>

void print_app_info()
{
#ifdef WELCOM
    printf("Welcom to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}