cmake_minimum_required(VERSION 3.15)
project(ffi)

add_subdirectory(struct_printer)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/build)

set(SOURCE main.cc)
add_executable(printer ${SOURCE})
add_dependencies(printer struct_printer)

target_include_directories(
    printer PUBLIC
    struct_printer/gen
)

get_target_property(STRUCT_PRINTER_DIR struct_printer LOCATION)
target_link_libraries(printer ${STRUCT_PRINTER_DIR}/libstruct_printer.so)