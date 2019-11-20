#![recursion_limit = "8192"]
#![feature(unboxed_closures)]

#[macro_use]
mod helpers;
mod args;
mod defs;

use crate::args::*;
use crate::defs::*;
use crate::helpers::*;
use libc::c_void;

#[cfg(not(target_os = "linux"))]
use ctor::ctor;

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

pub fn hook_titleinput(
    key: *mut c_void,
    dwgfx: *mut c_void,
    map: *mut c_void,
    game: *mut c_void,
    obj: *mut c_void,
    help: *mut c_void,
    music: *mut c_void,
) {
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

pub fn hook_gameinput(
    key: *mut c_void,
    dwgfx: *mut c_void,
    game: *mut c_void,
    map: *mut c_void,
    obj: *mut c_void,
    help: *mut c_void,
    music: *mut c_void,
) {
    let hascontrol_ptr = game.wrapping_offset(0x74) as *mut bool;
    let state_ptr = game.wrapping_offset(96) as *mut libc::c_int;
    let running_ptr = SCRIPT_GLOBAL.0.wrapping_offset(100) as *mut bool;
    let delay_ptr = SCRIPT_GLOBAL.0.wrapping_offset(96) as *mut libc::c_int;
    unsafe {
        if *state_ptr == 5000 {
            let delay = *delay_ptr;
            let hascontrol = *hascontrol_ptr;
            let running = *running_ptr;
            *delay_ptr = 0;
            *hascontrol_ptr = true;
            *running_ptr = false;
            HOOK_GAMEINPUT.call(key, dwgfx, game, map, obj, help, music);
            *delay_ptr = delay;
            *hascontrol_ptr = hascontrol;
            if *running_ptr == false {
                *running_ptr = running;
            }
        } else {
            HOOK_GAMEINPUT.call(key, dwgfx, game, map, obj, help, music);
        }
    }
}

#[cfg_attr(not(target_os = "linux"), ctor)]
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
    let titleinput = unsafe {
        get_symbol(b"_Z10titleinputR7KeyPollR8GraphicsR8mapclassR4GameR11entityclassR12UtilityClassR10musicclass")
    };
    let gameinput = unsafe {
        get_symbol(b"_Z9gameinputR7KeyPollR8GraphicsR4GameR8mapclassR11entityclassR12UtilityClassR10musicclass")
    };

    unsafe {
        hook(&HOOK_PHYSFS_INIT, *physfs_init, hook_physfs_init);
        hook(
            &HOOK_PRELOADER_RENDER,
            *preloader_render,
            hook_preloader_render,
        );
        hook(&HOOK_TITLEINPUT, *titleinput, hook_titleinput);
        hook(&HOOK_GAMEINPUT, *gameinput, hook_gameinput);
    }
}
