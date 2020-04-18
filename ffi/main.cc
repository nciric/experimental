#include "struct_printer.h"

#include <iostream>

int main() {
    PrintableStruct example;

    example.text_ptr = "Nebojsa Ciric";
    example.short_number = 4;
    example.long_number = 0x100000L;
    example.logical = false;

    uint32_t count = struct_printer(example);
    std::cout << "From C++, count elements: " << count << std::endl;

    return 0;
}