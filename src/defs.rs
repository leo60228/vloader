use detour::static_detour;

static_detour! {
    pub static PHYSFS_INIT: unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int;
}

dlsym! {
    pub static PHYSFS_permitSymbolicLinks: unsafe extern "C" fn(libc::c_int) = b"PHYSFS_permitSymbolicLinks";
}
