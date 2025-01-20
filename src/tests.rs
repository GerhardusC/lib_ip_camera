#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use crate::utils::log_buffer;

    #[test]
    fn buffer_logged_to_file() {
        let text = "Hello world!";
        let buffer = text.as_bytes();
        log_buffer(PathBuf::from_str("./log")
            .expect("Should be able to convert str to path."), buffer)
            .expect("Should be able to log buffer to file.");

        let file = fs::read("./log")
            .expect("File should exist.");

        // Test cleanup
        fs::remove_file("./log").unwrap();

        let contents = String::from_utf8(file)
            .expect("Buff should convert to string.");

        assert_eq!(text, &contents);
    }
}

