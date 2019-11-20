use crate::defs::*;
use std::os::raw::*;

fn hook_physfs_init(argv0: *mut c_char) -> c_int {
    unsafe {
        PHYSFS_permitSymbolicLinks(1);
        HOOK_PHYSFS_INIT.call(argv0)
    }
}

hook!(HOOK_PHYSFS_INIT @ b"PHYSFS_init" = hook_physfs_init);
