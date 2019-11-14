#![recursion_limit = "8192"]

use ctor::ctor;
use detour::static_detour;
use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::mem::transmute;

macro_rules! dlsym {
    ($name:literal) => {
        libc::dlsym(
            libc::RTLD_DEFAULT,
            concat!($name, "\0").as_ptr() as *const _,
        )
    };
    ($(static $id:ident: $ty:ty = $name:literal;)*) => {
        $(static $id: Lazy<$ty> = Lazy::new(|| unsafe {
            transmute(dlsym!($name))
        });)*
    };
}

extern "C" {
    static __progname: *const libc::c_char;
}

static_detour! {
    static PHYSFS_INIT: unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int;
}

dlsym! {
    static SYMLINKS: unsafe extern "C" fn(libc::c_int) = "PHYSFS_permitSymbolicLinks";
}

pub fn hook_physfs_init(argv0: *mut libc::c_char) -> libc::c_int {
    println!("PHYSFS_init({:?})", argv0);
    panic!("panic test");
    unsafe {
        SYMLINKS(1);
        PHYSFS_INIT.call(argv0)
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

    let physfs_init = unsafe { dlsym!("PHYSFS_init") };

    dbg!(physfs_init);

    assert!(!physfs_init.is_null());

    unsafe {
        PHYSFS_INIT.initialize(transmute(physfs_init), hook_physfs_init).unwrap();
        PHYSFS_INIT.enable().unwrap();
    }
}
