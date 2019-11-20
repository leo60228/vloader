use crate::defs::*;
use std::os::raw::*;

fn hook_titleinput(
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
        if let Some(idx) = libargs::args().get(1).and_then(|s| s.parse().ok()) {
            editorclass_getDirectoryData(ED_GLOBAL.0);
            *playcustomlevel_ptr = idx;
            scriptclass_startgamemode(SCRIPT_GLOBAL.0, 22, key, dwgfx, game, map, obj, help, music);
        } else {
            HOOK_TITLEINPUT.disable().unwrap();
            HOOK_TITLEINPUT.call(key, dwgfx, map, game, obj, help, music);
        }
    }
}

hook!(HOOK_TITLEINPUT @ b"_Z10titleinputR7KeyPollR8GraphicsR8mapclassR4GameR11entityclassR12UtilityClassR10musicclass" = hook_titleinput);
