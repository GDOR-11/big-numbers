use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::io::Result;

pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)
}
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory)?;
    }
    File::create(path)?.write_all(content.as_bytes())?;
    Ok(())
}
pub fn delete_path(path: &Path) -> Result<()> {
    if path.is_file() {
        fs::remove_file(path)
    } else {
        fs::remove_dir_all(path)
    }
}
