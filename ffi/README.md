## First pass in FFI C++->Rust

1. Using cbindgen library to auto-generate C++ header files from Rust code
2. Making cdylib instead of staticlib to avoid problems with clib differences
3. Not dealing with strings yet, just PODs
4. Not having cmake yet

To compile/build/run:

```
$cd struct_printer
$cargo build --release
$sudo cp target/release/libstruct_printer.so /usr/lib
$ldconfig -n -v /usr/lib
$cd ..
$clang++ -O2 -Wall -o printer -Istruct_printer/gen -Lstruct_printer/target/release main.cc -lstruct_printer
$./printer
```

Result should be:

```
Logical     : false
Short number: 4
Long number : 1048576
```