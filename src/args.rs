use std::path::PathBuf;

#[cfg(target_os = "linux")]
use {once_cell::sync::OnceCell, std::ffi::CStr};

#[cfg(not(target_os = "linux"))]
pub fn cmdline() -> Vec<String> {
    std::env::args().collect()
}

pub fn exe() -> PathBuf {
    PathBuf::from(cmdline().into_iter().next().unwrap())
}

#[cfg(target_os = "linux")]
static ARGV: OnceCell<Vec<String>> = OnceCell::new();

#[cfg(target_os = "linux")]
pub fn cmdline() -> Vec<String> {
    ARGV.get().unwrap().clone()
}

#[cfg(target_os = "linux")]
#[used]
#[link_section = ".init_array"]
#[no_mangle]
static SET_ARGV: [extern "C" fn(
    libc::c_int,
    *const *const libc::c_char,
    *const *const libc::c_char,
); 1] = {
    extern "C" fn set_argv(
        argc: libc::c_int,
        argv: *const *const libc::c_char,
        _env: *const *const libc::c_char,
    ) {
        ARGV.set(
            (0..argc)
                .map(|i| unsafe {
                    let cstr = CStr::from_ptr(*argv.offset(i as isize));
                    String::from_utf8(cstr.to_bytes().to_vec()).unwrap_or_else(|_| "".into())
                })
                .collect(),
        )
        .unwrap();
        crate::init();
    }

    [set_argv]
};
