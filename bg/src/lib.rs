// use js_sys::{Function, Object, Reflect, WebAssembly};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::{spawn_local, JsFuture};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn func() {
    console_log!("{}", "hi");
}

#[wasm_bindgen]
pub fn value() -> JsValue::Null {
    JsValue::Null
}
