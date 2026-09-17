use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let mut file = file;
    file.write_all(content.as_bytes()).unwrap();
}
