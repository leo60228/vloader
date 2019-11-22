use crate::helpers::SyncPtr;
use detour::static_detour;
use libc::c_void;

static_detour! {
    pub static HOOK_PHYSFS_INIT: unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int;
    pub static HOOK_PRELOADER_RENDER: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
    pub static HOOK_TITLEINPUT: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void);
    pub static HOOK_CUSTOMLOADQUICK: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void);
    pub static HOOK_GAMEINPUT: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void);
}

dlsym! {
    pub static PHYSFS_permitSymbolicLinks: unsafe extern "C" fn(libc::c_int) = b"PHYSFS_permitSymbolicLinks";
    pub static editorclass_getDirectoryData: unsafe extern "C" fn(*mut c_void) = b"_ZN11editorclass16getDirectoryDataEv";
    pub static musicclass_play: unsafe extern "C" fn(*mut c_void, libc::c_int) = b"_ZN10musicclass4playEi";
    pub static scriptclass_startgamemode: unsafe extern "C" fn(*mut c_void, libc::c_int, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void) = b"_ZN11scriptclass13startgamemodeEiR7KeyPollR8GraphicsR4GameR8mapclassR11entityclassR12UtilityClassR10musicclass";
    pub static ED_GLOBAL: SyncPtr<c_void> = b"ed";
    pub static SCRIPT_GLOBAL: SyncPtr<c_void> = b"script";
}
