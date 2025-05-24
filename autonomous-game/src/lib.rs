use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// utils
#[wasm_bindgen]
pub fn print(name: &str) {
    log(&format!("Hello, {}!", name));
}

// calling the function in JS
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_name = logPosition)]
    fn log_position(x: f32, y: f32);

    #[wasm_bindgen(js_name = requestConnect)]
    fn request_connect_();

    #[wasm_bindgen(js_name = requestDisconnect)]
    fn request_disconnect_();

    #[wasm_bindgen(js_name = requestPaidTransaction)]
    fn request_paid_transaction_();

    #[wasm_bindgen(js_name = emitGameStart)]
    fn emit_game_start_();
}

// have to wrap the extern function
pub fn console_log(s: &str) {
    log(s);
}

pub fn request_connect() {
    request_connect_();
}

pub fn request_disconnect() {
    request_disconnect_();
}

pub fn request_paid_transaction() {
    request_paid_transaction_();
}

pub  fn emit_game_start(){
    emit_game_start_()
}

lazy_static! {
    static ref SHARED_STATE: Mutex<SharedState> = Mutex::new(SharedState {
        balance: 0.0,
        sui_address: "".to_string(),
        is_paid: false
    });
}

pub struct SharedState {
    pub sui_address: String,
    pub balance: f32,
    pub is_paid: bool,
}

pub fn get_state() -> std::sync::MutexGuard<'static, SharedState> {
    SHARED_STATE.lock().expect("fail to get state")
}

#[wasm_bindgen]
pub fn set_player_balance(balance: f32) {
    get_state().balance = balance;
}

#[wasm_bindgen]
pub fn get_player_balance() -> f32 {
    get_state().balance
}

#[wasm_bindgen]
pub fn update_sui_address(sui_address: String) {
    get_state().sui_address = sui_address;
}

#[wasm_bindgen]
pub fn get_sui_address() -> String {
    get_state().sui_address.clone()
}

#[wasm_bindgen]
pub fn update_is_paid(is_paid: bool) {
    get_state().is_paid = is_paid;
}

#[wasm_bindgen]
pub fn get_is_paid() -> bool {
    get_state().is_paid
}
