use libloading::os::unix::Library as UnixLibrary;
use libloading::Library;
use libloading::Symbol;
use once_cell::sync::Lazy;
use procfs::process::Process;
use std::path::PathBuf;

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

pub fn cmdline() -> Vec<String> {
    Process::myself().unwrap().cmdline().unwrap()
}

pub fn exe() -> PathBuf {
    Process::myself().unwrap().exe().unwrap()
}
