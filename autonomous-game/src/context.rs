use thread_assert::set_thread_id;

pub static mut GAME_STATE: Option<GameContext> = None;

#[derive(Default, Debug, Clone)]
pub struct GameContext {
    pub sui_address: String,
}

/// Initialize a new game context. Should be called once at startup.
/// # Panics
/// Panics if called multiple times or from different threads.
pub fn new_context() {
    set_thread_id();

    thread_assert::same_thread();
    unsafe {
        if GAME_STATE.is_some() {
            panic!("Game context already initialized!");
        }
        GAME_STATE = Some(GameContext::default());
    }
}

/// Get a mutable reference to the game context.
/// # Panics
/// Panics if context is not initialized or called from different thread.
pub fn get_context() -> &'static mut GameContext {
    thread_assert::same_thread();

    unsafe {
        GAME_STATE
            .as_mut()
            .expect("Game context not initialized! Call new_context() first.")
    }
}

pub(crate) mod thread_assert {
    use std::sync::Once;
    static mut THREAD_ID: Option<std::thread::ThreadId> = None;
    static INIT: Once = Once::new();

    /// Set the main thread ID. Should be called once at program start.
    pub fn set_thread_id() {
        INIT.call_once(|| unsafe {
            THREAD_ID = Some(std::thread::current().id());
        });
    }

    /// Verify that the current thread is the main thread.
    /// # Panics
    /// Panics if called from a different thread or if thread ID wasn't set.
    pub fn same_thread() {
        unsafe {
            thread_local! {
                static CURRENT_THREAD_ID: std::thread::ThreadId = std::thread::current().id();
            }

            match THREAD_ID {
                None => panic!("Thread ID not initialized! Call set_thread_id() first."),
                Some(main_thread) => {
                    assert_eq!(
                        main_thread,
                        CURRENT_THREAD_ID.with(|id| *id),
                        "GameContext accessed from wrong thread!"
                    );
                }
            }
        }
    }
}
