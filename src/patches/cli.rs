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
    let fademode_ptr = dwgfx.wrapping_offset(840) as *mut c_int;
    let args: Vec<c_int> = std::env::args()
        .into_iter()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();
    unsafe {
        if args.len() == 0 {
            HOOK_TITLEINPUT.disable().unwrap();
            HOOK_TITLEINPUT.call(key, dwgfx, map, game, obj, help, music);
        } else if args.len() == 1 || args.len() == 6 || args.len() == 7 {
            let gamemode = if args.len() == 1 { 22 } else { 23 };
            let idx = args[0];
            editorclass_getDirectoryData(ED_GLOBAL.0);
            *playcustomlevel_ptr = idx;
            scriptclass_startgamemode(
                SCRIPT_GLOBAL.0,
                gamemode,
                key,
                dwgfx,
                game,
                map,
                obj,
                help,
                music,
            );

            if args.len() >= 6 {
                *fademode_ptr = 0;
            }

            if args.len() == 7 {
                musicclass_play(music, args[6]);
            }
        } else {
            log::error!("Bad number of arguments: {}", args.len());
            std::process::exit(64); // EX_USAGE from sysexits.h
        }
    }
}

fn hook_customloadquick(
    this: *mut c_void,
    level: *mut c_void,
    map: *mut c_void,
    obj: *mut c_void,
    music: *mut c_void,
) {
    unsafe {
        HOOK_CUSTOMLOADQUICK.call(this, level, map, obj, music);
    }
    let args: Vec<c_int> = std::env::args()
        .into_iter()
        .skip(1)
        .flat_map(|x| x.parse())
        .collect();
    if args.len() >= 6 {
        let savex_ptr = this.wrapping_offset(48) as *mut c_int;
        let savey_ptr = this.wrapping_offset(52) as *mut c_int;
        let saverx_ptr = this.wrapping_offset(56) as *mut c_int;
        let savery_ptr = this.wrapping_offset(60) as *mut c_int;
        let savegc_ptr = this.wrapping_offset(64) as *mut c_int;

        let savex = args[1];
        let savey = args[2];
        let saverx = args[3];
        let savery = args[4];
        let savegc = args[5];

        log::info!(
            "starting custom level at room ({}, {}) position ({}, {})",
            saverx,
            savery,
            savex,
            savey
        );

        unsafe {
            *saverx_ptr = saverx;
            *savery_ptr = savery;
            *savex_ptr = savex;
            *savey_ptr = savey;
            *savegc_ptr = savegc;
        }
    }
}

hook!(HOOK_TITLEINPUT @ b"_Z10titleinputR7KeyPollR8GraphicsR8mapclassR4GameR11entityclassR12UtilityClassR10musicclass" = hook_titleinput);
hook!(HOOK_CUSTOMLOADQUICK @ b"_ZN4Game15customloadquickESsR8mapclassR11entityclassR10musicclass" = hook_customloadquick);
