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

pub fn rtld_default() -> Library {
    unsafe { UnixLibrary::from_raw(libc::RTLD_DEFAULT) }.into()
}

static RTLD_DEFAULT: Lazy<Library> = Lazy::new(rtld_default);

pub unsafe fn get_symbol<T>(symbol: &[u8]) -> Symbol<'static, T> {
    RTLD_DEFAULT.get(symbol).unwrap()
}

pub unsafe fn hook<T, D>(detour: &StaticDetour<T>, func: T, hook: D)
where
    T: Function,
    D: Fn<T::Arguments, Output = T::Output> + Send + 'static,
{
    detour.initialize(func, hook).unwrap();
    detour.enable().unwrap();
}

pub fn exe() -> PathBuf {
    PathBuf::from(libargs::args().into_iter().next().unwrap())
}
