use std::os::raw::*;

pub mod pdelay;

pub type GamestateCallback = unsafe fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void);

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum GamestateTime {
    PreInput,
    PostInput,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Gamestate {
    pub num: c_int,
    pub time: GamestateTime,
    pub callback: GamestateCallback,
}

impl Gamestate {
    pub fn new(num: c_int, time: GamestateTime, callback: GamestateCallback) -> Self {
        Self { num, time, callback }
    }
}

inventory::collect!(Gamestate);
