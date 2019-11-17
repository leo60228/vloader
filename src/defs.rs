use detour::static_detour;
use libc::c_void;

static_detour! {
    pub static HOOK_PHYSFS_INIT: unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int;
    pub static HOOK_PRELOADER_RENDER: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
}

dlsym! {
    pub static PHYSFS_permitSymbolicLinks: unsafe extern "C" fn(libc::c_int) = b"PHYSFS_permitSymbolicLinks";
}
