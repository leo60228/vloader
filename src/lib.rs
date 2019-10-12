#![recursion_limit = "8192"]

use ctor::ctor;
use detour::static_detour;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use shrinkwraprs::Shrinkwrap;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::sync::atomic::{AtomicBool, Ordering};

extern "C" {
    static __progname: *const libc::c_char;
}

static_detour! {
    static GFX_LOAD: unsafe extern "C" fn(*mut Graphics);
    static GOTOROOM: unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int, *mut libc::c_void, *mut libc::c_void, *mut libc::c_void, *mut libc::c_void);
    static UNPACK_BINARY: unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool;
}

static CUSTOM: AtomicBool = AtomicBool::new(false);
static GRAPHICS: Lazy<Mutex<Option<GfxPointer>>> = Lazy::new(Default::default);
static CUSTOMPATH: Lazy<Mutex<String>> = Lazy::new(Default::default);

static MAKETILEARRAY: Lazy<unsafe extern "C" fn(*mut Graphics)> = Lazy::new(|| unsafe {
    transmute(libc::dlsym(
        libc::RTLD_DEFAULT,
        "_ZN8Graphics13MakeTileArrayEv\0".as_ptr() as *const _,
    ))
});
static MUSICCLASS: Lazy<unsafe extern "C" fn(*mut libc::c_void)> = Lazy::new(|| unsafe {
    transmute(libc::dlsym(
        libc::RTLD_DEFAULT,
        "_ZN10musicclassC1Ev\0".as_ptr() as *const _,
    ))
});
static LOADIMAGE: Lazy<unsafe extern "C" fn(*const libc::c_char, bool, bool) -> *mut libc::c_void> =
    Lazy::new(|| unsafe {
        transmute(libc::dlsym(
            libc::RTLD_DEFAULT,
            "_Z9LoadImagePKcbb\0".as_ptr() as *const _,
        ))
    });
static ISDIRECTORY: Lazy<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int> =
    Lazy::new(|| unsafe {
        transmute(libc::dlsym(
            libc::RTLD_DEFAULT,
            "PHYSFS_isDirectory\0".as_ptr() as *const _,
        ))
    });
static EXISTS: Lazy<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int> =
    Lazy::new(|| unsafe {
        transmute(libc::dlsym(
            libc::RTLD_DEFAULT,
            "PHYSFS_exists\0".as_ptr() as *const _,
        ))
    });

#[derive(Shrinkwrap, Debug, Clone, Copy)]
pub struct GfxPointer(*mut Graphics);

unsafe impl Send for GfxPointer {}
unsafe impl Sync for GfxPointer {}

#[derive(Debug)]
#[repr(C)]
pub struct Graphics {
    pub im_tiles: *mut libc::c_void,
    pub im_tiles2: *mut libc::c_void,
    pub im_tiles3: *mut libc::c_void,
    pub im_entcolours: *mut libc::c_void,
    pub im_sprites: *mut libc::c_void,
    pub im_flipsprites: *mut libc::c_void,
    pub im_bfont: *mut libc::c_void,
    pub im_bfontmask: *mut libc::c_void,
    pub im_teleporter: *mut libc::c_void,
    pub im_image0: *mut libc::c_void,
    pub im_image1: *mut libc::c_void,
    pub im_image2: *mut libc::c_void,
    pub im_image3: *mut libc::c_void,
    pub im_image4: *mut libc::c_void,
    pub im_image5: *mut libc::c_void,
    pub im_image6: *mut libc::c_void,
    pub im_image7: *mut libc::c_void,
    pub im_image8: *mut libc::c_void,
    pub im_image9: *mut libc::c_void,
    pub im_image10: *mut libc::c_void,
    pub im_image11: *mut libc::c_void,
    pub im_image12: *mut libc::c_void,
}

pub unsafe fn load_image(path: &CStr, no_blend: bool, no_alpha: bool) -> *mut libc::c_void {
    (LOADIMAGE)(path.as_ptr() as _, no_blend, no_alpha)
}

pub fn hook_gfx(this: *mut Graphics) {
    if let Some(mut lock) = GRAPHICS.try_lock() {
        *lock = Some(GfxPointer(this));
    }

    unsafe {
        if CUSTOM.load(Ordering::SeqCst) {
            let tiles_path =
                CString::new(format!("{}graphics/tiles.png", CUSTOMPATH.lock())).unwrap();
            let tiles2_path =
                CString::new(format!("{}graphics/tiles2.png", CUSTOMPATH.lock())).unwrap();
            if (EXISTS)(tiles_path.as_ptr() as *const _) != 0 {
                println!("redirecting tiles.png to {:?}", tiles_path);
                (*this).im_tiles = load_image(&tiles_path, false, false);
            }
            if (EXISTS)(tiles2_path.as_ptr() as *const _) != 0 {
                println!("redirecting tiles.png to {:?}", tiles2_path);
                (*this).im_tiles2 = load_image(&tiles2_path, false, false);
            }
        } else {
            GFX_LOAD.call(this);
        }
    }

    //println!("Graphics! {:?}", unsafe { &*this });
}

pub fn hook_unpack_binary(this: *mut libc::c_void, path: *const libc::c_char) -> bool {
    if !CUSTOM.load(Ordering::SeqCst) {
        unsafe {
            return UNPACK_BINARY.call(this, path);
        }
    }

    let redir_path = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let redir_path = CString::new(format!("{}{}", CUSTOMPATH.lock(), redir_path)).unwrap();

    unsafe {
        if (EXISTS)(redir_path.as_ptr() as *const _) != 0 {
            println!("redirecting music to {:?}", redir_path);
            UNPACK_BINARY.call(this, redir_path.as_ptr() as *const _)
        } else {
            println!(
                "{:?} missing, keeping path untouched as {:?}",
                redir_path,
                CStr::from_ptr(path)
            );
            UNPACK_BINARY.call(this, path)
        }
    }
}

pub fn hook_gotoroom(
    this: *mut libc::c_void,
    rx: libc::c_int,
    ry: libc::c_int,
    dwgfx: *mut libc::c_void,
    game: *mut libc::c_void,
    entity: *mut libc::c_void,
    music: *mut libc::c_void,
) {
    let mut custom = unsafe { (this as *mut u8).offset(306).read() } != 0;
    let customlevel = if custom {
        Some(
            unsafe { CStr::from_ptr(*(game.offset(1656) as *mut *const _)) }
                .to_str()
                .unwrap()
                .trim_end_matches(".vvvvvv")
                .to_owned()
                + "/",
        )
    } else {
        None
    };
    println!("{:?}", customlevel);
    if let Some(customlevel) = customlevel {
        if unsafe { (ISDIRECTORY)(CString::new(customlevel.clone()).unwrap().as_ptr()) } != 0 {
            *CUSTOMPATH.lock() = customlevel;
        } else {
            custom = false;
        }
    }

    let needs_reload = if custom {
        !CUSTOM.compare_and_swap(false, true, Ordering::SeqCst)
    } else {
        CUSTOM.compare_and_swap(true, false, Ordering::SeqCst)
    };

    if needs_reload {
        println!("reloading gfx");
        unsafe {
            let gfx = *GRAPHICS.lock().unwrap() as *mut u8;
            hook_gfx(gfx as *mut _);
            gfx.offset(0x120).write_bytes(0, 0x30);
            println!("zeroed out arrays");
            (MAKETILEARRAY)(gfx as *mut _);
            println!("built array");
            println!("reloading music");
            (MUSICCLASS)(music);
            println!("reloaded music");
        }
    }

    unsafe {
        GOTOROOM.call(this, rx, ry, dwgfx, game, entity, music);
    }
}

#[ctor]
fn init() {
    let progname = unsafe { CStr::from_ptr(__progname) };
    println!("{:?}", progname);
    if progname
        .to_str()
        .map(|s| s != "vvvvvv.x86_64")
        .unwrap_or(false)
    {
        return;
    }

    let gfx_load = unsafe {
        libc::dlsym(
            libc::RTLD_DEFAULT,
            "_ZN17GraphicsResourcesC2Ev\0".as_ptr() as *const _,
        )
    };
    let gotoroom = unsafe {
        libc::dlsym(
            libc::RTLD_DEFAULT,
            "_ZN8mapclass8gotoroomEiiR8GraphicsR4GameR11entityclassR10musicclass\0".as_ptr()
                as *const _,
        )
    };
    let unpack_binary = unsafe {
        libc::dlsym(
            libc::RTLD_DEFAULT,
            "_ZN10binaryBlob12unPackBinaryEPKc\0".as_ptr() as *const _,
        )
    };

    dbg!(gfx_load);
    dbg!(gotoroom);
    dbg!(unpack_binary);

    assert!(!gfx_load.is_null());
    assert!(!gotoroom.is_null());
    assert!(!unpack_binary.is_null());

    unsafe {
        GFX_LOAD.initialize(transmute(gfx_load), hook_gfx).unwrap();
        GFX_LOAD.enable().unwrap();
        GOTOROOM
            .initialize(transmute(gotoroom), hook_gotoroom)
            .unwrap();
        GOTOROOM.enable().unwrap();
        UNPACK_BINARY
            .initialize(transmute(unpack_binary), hook_unpack_binary)
            .unwrap();
        UNPACK_BINARY.enable().unwrap();
    }
}
