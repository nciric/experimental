# struct_printer/CMakeLists.txt

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "target/debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "target/release")
endif ()

# This is where Cargo build result is. We'll link to it from parent.
set(STRUCT_PRINTER_SO_DIR "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/")

# This is a preset env variable for Cargo to output build there.
set(CARGO_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}")

add_custom_target(struct_printer ALL
    COMMENT "Compiling struct_printer module"
    COMMAND RUST_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(struct_printer PROPERTIES LOCATION ${STRUCT_PRINTER_SO_DIR})