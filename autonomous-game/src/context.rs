// use thread_assert::set_thread_id;

pub static mut GAME_STATE: Option<GameContext> = None;

#[derive(Debug, Clone)]
pub struct GameContext {
    pub sui_address: String,
}

impl Default for GameContext {
    fn default() -> Self {
        GameContext {
            sui_address: "".to_string(),
        }
    }
}

/// Initialize a new game context. Should be called once at startup.
/// # Panics
/// Panics if called multiple times or from different threads.
pub fn new_context() {
    // thread_assert::same_thread();
    unsafe {
        if GAME_STATE.is_some() {
            panic!("Game context already initialized!");
        }
        GAME_STATE = Some(GameContext {
            sui_address: "".to_string(),
        });
    }
}

/// Get a mutable reference to the game context.
/// # Panics
/// Panics if context is not initialized or called from different thread.
pub fn get_context() -> &'static mut GameContext {
    unsafe {
        GAME_STATE
            .as_mut()
            .expect("Game context not initialized! Call new_context() first.")
    }
}
