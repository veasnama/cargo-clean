use std::fs::read_dir;
use std::io::Error;
use std::path::{Path, PathBuf};
pub fn get_dirs<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Error> {
    let entries = read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, Error>>();
    match entries {
        Ok(vector_path) => Ok(vector_path),
        Err(e) => Err(e),
    }
}
