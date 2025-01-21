#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use crate::utils::log_buffer;
    use crate::camera_control::{CameraControl, Direction};

    #[test]
    fn buffer_logged_to_file() {
        let text = "Hello world!";
        let buffer = text.as_bytes();
        log_buffer(&PathBuf::from_str("./log")
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
    fn cant_move_camera_with_invalid_ip() {
        let mut control = CameraControl::new("192.168.0.5", 554);
        control.connect();
        assert_eq!(1, 2);
    }

}

