use lazy_static::lazy_static;
use std::sync::Mutex;
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

pub fn console_log(s: &str) {
    log(s);
}

lazy_static! {
    static ref SHARED_STATE: Mutex<SharedState> = Mutex::new(SharedState {
        speed: 0.0,
        sui_address: "".to_string(),
    });
}

pub struct SharedState {
    pub speed: f32,
    pub sui_address: String,
}

pub fn get_state() -> std::sync::MutexGuard<'static, SharedState> {
    SHARED_STATE.lock().expect("fail to get state")
}

#[wasm_bindgen]
pub fn set_player_speed(speed: f32) {
    get_state().speed = speed;
}

#[wasm_bindgen]
pub fn get_player_speed() -> f32 {
    get_state().speed
}

#[wasm_bindgen]
pub fn update_sui_address(sui_address: String) {
    get_state().sui_address = sui_address;
}

#[wasm_bindgen]
pub fn get_sui_address() -> String {
    get_state().sui_address.clone()
}
