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
