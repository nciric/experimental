I've used pre 0.1 ICU4X icu_locale crate to test Wasm use and size.

Follow [MDN](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) article about compiling Rust to Wasm.

The web site using npm and nodejs is in this repo under *locale-wasm-site* folder.

Results:
1. Rust -> Wasm -> Html/JS works with minor wrapping of the code!
2. Original size of wasm file (default opt) was: wasm_locale_bg.wasm **31.8** KiB
3. Using aggresive size optimization, we got modest gains: **28.7** KiB
4. Further optimization using wasm-opt util from [binaryen](https://github.com/WebAssembly/binaryen) saved ~**50** bytes.
5. Gzipping the wasm file drops the size to **12.1** KiB
6. Speed impact was not measured
