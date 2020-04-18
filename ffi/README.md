## First pass in FFI C++->Rust

1. Using cbindgen library to auto-generate C++ header files from Rust code
2. Making cdylib instead of staticlib to avoid problems with clib differences
3. Not dealing with strings yet, just PODs
4. Not having cmake yet

To compile/build/run:

```
$cd ffi
$mkdir build (once)
$cmake -DCMAKE_BUILD_TYPE=Release|Debug
$make
$build/printer
```

Result should be:

```
Logical     : false
Short number: 4
Long number : 1048576
```

Right now, installation is not automatic:

```
$sudo cp target/release/libstruct_printer.so /usr/lib
$ldconfig -n -v /usr/lib
```