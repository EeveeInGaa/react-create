use std::{fs, io};
use std::path::Path;

pub fn write_new_file(path: &Path, content: &str) -> io::Result<()> {
    if path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("File already exists: {}", path.display()),
        ));
    }

    fs::write(path, content)
}