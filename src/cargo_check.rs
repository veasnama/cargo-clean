use std::ffi::OsStr;
use std::path::Path;
use std::{fs, io::Error};
#[allow(dead_code)]
pub fn is_cargo_file<P: AsRef<Path>>(path: P) -> Result<Vec<bool>, Error> {
    let mut is_cargo: Vec<bool> = vec![];
    let path = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;

    for p in path {
        if p.file_name() == Some(OsStr::new("Cargo.toml")) {
            is_cargo.push(true);
        } else {
            is_cargo.push(false);
        }
    }
    Ok(vec![true])
}
