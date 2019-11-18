use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
pub fn exe() -> PathBuf {
    fs::read_link("/proc/self/exe").unwrap()
}

#[cfg(target_os = "linux")]
pub fn cmdline() -> Vec<String> {
    fs::read_to_string("/proc/self/cmdline")
        .unwrap()
        .split('\0')
        .map(From::from)
        .collect()
}

#[cfg(not(target_os = "linux"))]
pub fn cmdline() -> Vec<String> {
    std::env::args().collect()
}

#[cfg(not(target_os = "linux"))]
pub fn exe() -> PathBuf {
    PathBuf::from(std::env::args().next().unwrap())
}
