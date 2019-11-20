use crate::defs::*;
use std::os::raw::*;

fn hook_preloader_render(_dwgfx: *mut c_void, game: *mut c_void, _help: *mut c_void) {
    let gamestate_ptr = game.wrapping_offset(112) as *mut libc::c_int;
    unsafe {
        *gamestate_ptr = 1;
    }
}

hook!(HOOK_PRELOADER_RENDER @ b"_Z15preloaderrenderR8GraphicsR4GameR12UtilityClass" = hook_preloader_render);
