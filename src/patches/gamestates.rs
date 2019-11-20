use crate::defs::*;
use crate::gamestates::*;
use std::os::raw::*;

fn hook_gameinput(
    key: *mut c_void,
    dwgfx: *mut c_void,
    game: *mut c_void,
    map: *mut c_void,
    obj: *mut c_void,
    help: *mut c_void,
    music: *mut c_void,
) {
    unsafe {
        let state_ptr = game.wrapping_offset(96) as *mut libc::c_int;
        let state = *state_ptr;

        for s in inventory::iter::<Gamestate> {
            if s.num == state && s.time == GamestateTime::PreInput {
                (s.callback)(key, dwgfx, game, map, obj, help, music);
            }
        }

        HOOK_GAMEINPUT.call(key, dwgfx, game, map, obj, help, music);

        for s in inventory::iter::<Gamestate> {
            if s.num == state && s.time == GamestateTime::PostInput {
                (s.callback)(key, dwgfx, game, map, obj, help, music);
            }
        }
    }
}

hook!(HOOK_GAMEINPUT @ b"_Z9gameinputR7KeyPollR8GraphicsR4GameR8mapclassR11entityclassR12UtilityClassR10musicclass" = hook_gameinput);
