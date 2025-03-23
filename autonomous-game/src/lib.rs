pub mod context;
use context::get_context;
use wasm_bindgen::prelude::*;

// utils
#[wasm_bindgen]
pub fn print(name: &str) {
    log(&format!("Hello, {}!", name));
}

// calling the functino in JS
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_name = logPosition)]
    fn log_position(x: f32, y: f32);
}

pub fn log_player_position(x: f32, y: f32) {
    log_position(x, y);
}

#[wasm_bindgen]
pub fn update_sui_address(sui_address: String) {
    get_context().sui_address = sui_address;
}

#[wasm_bindgen]
pub fn get_sui_address() -> String {
    get_context().sui_address.clone()
}
