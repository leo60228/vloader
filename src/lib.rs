#![recursion_limit = "8192"]
#![feature(unboxed_closures)]

#[macro_use]
mod helpers;
mod defs;

use crate::defs::*;
use crate::helpers::*;
use ctor::ctor;
use libc::c_void;

pub fn hook_physfs_init(argv0: *mut libc::c_char) -> libc::c_int {
    println!("PHYSFS_init({:?})", argv0);
    unsafe {
        PHYSFS_permitSymbolicLinks(1);
        HOOK_PHYSFS_INIT.call(argv0)
    }
}

pub fn hook_preloader_render(_dwgfx: *mut c_void, game: *mut c_void, _help: *mut c_void) {
    let gamestate_ptr = game.wrapping_offset(112) as *mut libc::c_int;
    unsafe {
        *gamestate_ptr = 1;
    }
}

pub fn hook_titleinput(key: *mut c_void, dwgfx: *mut c_void, map: *mut c_void, game: *mut c_void, obj: *mut c_void, help: *mut c_void, music: *mut c_void) {
    let playcustomlevel_ptr = game.wrapping_offset(1640) as *mut libc::c_int;
    unsafe {
        if let Some(idx) = cmdline().get(1).and_then(|s| s.parse().ok()) {
            editorclass_getDirectoryData(ED_GLOBAL.0);
            *playcustomlevel_ptr = idx;
            scriptclass_startgamemode(SCRIPT_GLOBAL.0, 22, key, dwgfx, game, map, obj, help, music);
        } else {
            HOOK_TITLEINPUT.disable().unwrap();
            HOOK_TITLEINPUT.call(key, dwgfx, map, game, obj, help, music);
        }
    }
}

#[ctor]
fn init() {
    let exe = exe();
    let progname = exe.file_name().unwrap();
    println!("{:?}", progname);
    if progname != "vvvvvv.x86_64" {
        return;
    }

    println!("{:?}", cmdline());

    let physfs_init = unsafe { get_symbol(b"PHYSFS_init") };
    let preloader_render =
        unsafe { get_symbol(b"_Z15preloaderrenderR8GraphicsR4GameR12UtilityClass") };
    let titleinput =
        unsafe { get_symbol(b"_Z10titleinputR7KeyPollR8GraphicsR8mapclassR4GameR11entityclassR12UtilityClassR10musicclass") };


    unsafe {
        hook(&HOOK_PHYSFS_INIT, *physfs_init, hook_physfs_init);
        hook(&HOOK_PRELOADER_RENDER, *preloader_render, hook_preloader_render);
        hook(&HOOK_TITLEINPUT, *titleinput, hook_titleinput);
    }
}
