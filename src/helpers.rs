use detour::{Function, StaticDetour};
use libloading::os::unix::Library as UnixLibrary;
use libloading::Library;
use libloading::Symbol;
use once_cell::sync::Lazy;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct SyncPtr<T: ?Sized>(pub *mut T);

unsafe impl<T: ?Sized> Send for SyncPtr<T> {}
unsafe impl<T: ?Sized> Sync for SyncPtr<T> {}

macro_rules! dlsym {
    ($($vis:vis static $id:ident: $ty:ty = $name:literal;)*) => {
        $(
            #[allow(non_upper_case_globals)]
            $vis static $id: ::once_cell::sync::Lazy<::libloading::Symbol<'static, $ty>> = ::once_cell::sync::Lazy::new(|| unsafe {
                $crate::helpers::get_symbol($name)
            });
        )*
    };
}

macro_rules! hook {
    ($hook:ident @ $sym:literal = $func:expr) => {
        #[::ctor::ctor]
        unsafe fn apply_hook() {
            if !$crate::helpers::is_v6() {
                return;
            }
            println!(
                "hooking {} with {}",
                ::cpp_demangle::Symbol::new($sym as &[u8])
                    .map(|x| x.to_string())
                    .unwrap_or_else(|_| String::from_utf8_lossy($sym).into()),
                stringify!($func)
            );
            $crate::helpers::hook(&$hook, $sym, $func);
        }
    };
}

pub fn rtld_default() -> Library {
    unsafe { UnixLibrary::from_raw(libc::RTLD_DEFAULT) }.into()
}

static RTLD_DEFAULT: Lazy<Library> = Lazy::new(rtld_default);

pub unsafe fn get_symbol<T>(symbol: &[u8]) -> Symbol<'static, T> {
    RTLD_DEFAULT.get(symbol).unwrap()
}

pub unsafe fn hook<T, D>(detour: &StaticDetour<T>, func: &[u8], hook: D)
where
    T: Function,
    D: Fn<T::Arguments, Output = T::Output> + Send + 'static,
{
    detour.initialize(*get_symbol(func), hook).unwrap();
    detour.enable().unwrap();
}

pub fn exe() -> PathBuf {
    PathBuf::from(libargs::args().into_iter().next().unwrap())
}

pub fn is_v6() -> bool {
    static IS_V6: Lazy<bool> = Lazy::new(|| {
        exe()
            .file_name()
            .map(|x| x == "vvvvvv.x86_64")
            .unwrap_or(false)
    });
    *IS_V6
}
