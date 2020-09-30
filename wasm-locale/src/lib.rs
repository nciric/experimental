use wasm_bindgen::prelude::*;
use icu_locale::Locale;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn canonicalize_locale(locale: &str) {
    let loc: Locale = locale.parse()
        .expect("Failed to parse.");
    alert(&format!("Canonicalized locale: {}", loc));
}
