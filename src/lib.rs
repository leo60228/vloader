#![recursion_limit = "8192"]

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

    dbg!(*physfs_init);

    unsafe {
        HOOK_PHYSFS_INIT
            .initialize(*physfs_init, hook_physfs_init)
            .unwrap();
        HOOK_PHYSFS_INIT.enable().unwrap();
        HOOK_PRELOADER_RENDER
            .initialize(*preloader_render, hook_preloader_render)
            .unwrap();
        HOOK_PRELOADER_RENDER.enable().unwrap();
    }
}
