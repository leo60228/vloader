use super::*;
use crate::defs::*;
use crate::state::*;

pub unsafe fn pdelay_pre(
    _key: *mut c_void,
    _dwgfx: *mut c_void,
    game: *mut c_void,
    _map: *mut c_void,
    _obj: *mut c_void,
    _help: *mut c_void,
    _music: *mut c_void,
) {
    let mut state = get_state();

    let hascontrol_ptr = game.wrapping_offset(0x74) as *mut bool;
    let running_ptr = SCRIPT_GLOBAL.0.wrapping_offset(100) as *mut bool;
    let delay_ptr = SCRIPT_GLOBAL.0.wrapping_offset(96) as *mut libc::c_int;

    state.delay = *delay_ptr;
    state.hascontrol = *hascontrol_ptr;
    state.running = *running_ptr;
    *delay_ptr = 0;
    *hascontrol_ptr = true;
    *running_ptr = false;
}

pub unsafe fn pdelay_post(
    _key: *mut c_void,
    _dwgfx: *mut c_void,
    game: *mut c_void,
    _map: *mut c_void,
    _obj: *mut c_void,
    _help: *mut c_void,
    _music: *mut c_void,
) {
    let state = get_state();

    let hascontrol_ptr = game.wrapping_offset(0x74) as *mut bool;
    let running_ptr = SCRIPT_GLOBAL.0.wrapping_offset(100) as *mut bool;
    let delay_ptr = SCRIPT_GLOBAL.0.wrapping_offset(96) as *mut libc::c_int;

    *delay_ptr = state.delay;
    *hascontrol_ptr = state.hascontrol;
    if *running_ptr == false {
        *running_ptr = state.running;
    }
}

inventory::submit! {
    Gamestate::new(5000, GamestateTime::PreInput, pdelay_pre)
}

inventory::submit! {
    Gamestate::new(5000, GamestateTime::PostInput, pdelay_post)
}
