use crate::*;

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn update() -> Self {
        env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"))
    }
}
