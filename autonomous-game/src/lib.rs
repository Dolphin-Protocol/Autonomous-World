use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
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
