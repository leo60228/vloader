#![recursion_limit = "8192"]

#[macro_use]
mod helpers;
mod defs;

use crate::defs::*;
use crate::helpers::*;
use ctor::ctor;

pub fn hook_physfs_init(argv0: *mut libc::c_char) -> libc::c_int {
    println!("PHYSFS_init({:?})", argv0);
    unsafe {
        PHYSFS_permitSymbolicLinks(1);
        PHYSFS_INIT.call(argv0)
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

    dbg!(*physfs_init);

    unsafe {
        PHYSFS_INIT
            .initialize(*physfs_init, hook_physfs_init)
            .unwrap();
        PHYSFS_INIT.enable().unwrap();
    }
}
