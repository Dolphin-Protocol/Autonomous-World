use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = logPosition)]
    fn log_position(x: f32, y: f32);
}

pub fn log_player_position(x: f32, y: f32) {
    log_position(x, y);
}

#[wasm_bindgen]
pub struct GameState {
    pub speed: f32,
}


impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { speed: 0.0 }
    }

    #[wasm_bindgen]
    pub fn set_player_speed(&mut self, speed: f32) {
            self.speed = speed;
    }

    #[wasm_bindgen]
    pub fn get_player_speed(&self) -> f32 {
        self.speed
    }
}
