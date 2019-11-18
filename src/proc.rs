use std::fs;
use std::path::PathBuf;

pub fn exe() -> PathBuf {
    fs::read_link("/proc/self/exe").unwrap()
}

pub fn cmdline() -> Vec<String> {
    fs::read_to_string("/proc/self/cmdline")
        .unwrap()
        .split('\0')
        .map(From::from)
        .collect()
}
