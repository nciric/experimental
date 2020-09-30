const js = import("./node_modules/wasm-locale/wasm_locale.js");
js.then(js => {
  js.canonicalize_locale("sr-cyrl-rs-u-kn-true");
});