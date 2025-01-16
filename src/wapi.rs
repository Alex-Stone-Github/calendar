use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_name = alert)]
    pub fn alert(s: &str);
}
