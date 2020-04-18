#include "struct_printer.h"

int main() {
    PrintableStruct example;

    example.short_number = 4;
    example.long_number = 0x100000L;
    example.logical = false;

    struct_printer(example);

    return 0;
}