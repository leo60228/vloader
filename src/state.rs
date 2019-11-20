use once_cell::sync::Lazy;
use std::os::raw::*;
use std::sync::{Mutex, MutexGuard};

#[derive(Default, Clone)]
pub struct State {
    pub delay: c_int,
    pub hascontrol: bool,
    pub running: bool,
}

static STATE: Lazy<Mutex<State>> = Lazy::new(Default::default);

pub fn get_state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}
