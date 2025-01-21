use std::{fs::{self, DirBuilder}, io::Write, path::PathBuf};

use crate::error::Error;

pub fn log_buffer(path: &PathBuf, buff: &[u8]) -> Result<(), Error> {
    let file = fs::OpenOptions::new().append(true).write(true).open(&path);
    match file {
        Ok(mut file) => {
            file.write_all(&buff)?;
            return Ok(());
        }
        Err(_e) => {
            if let Ok(mut file) = fs::File::create(&path) {
                file.write_all(&buff)?;
                return Ok(());
            }
        }
    }

    if let Some(dirname) = path.parent() {
        DirBuilder::new().recursive(true).create(dirname)?;
        return log_buffer(path, &buff);
    }
    Err(Error::LogWriterError)
}