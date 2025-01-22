#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use crate::utils::log_buffer;
    use crate::camera_control::{CameraControl, Direction};
    use crate::error::Error;

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

    #[test]
    fn cannot_connect_to_ip() {
        let mut control = CameraControl::new("192.168.0.23", 554);
        let res = control.connect();
        match res {
            Ok(x) => {
                println!("CONNECTED... {:?}", x);
                panic!();
            },
            Err(e) => {
                match e {
                    Error::IoError(_error) => {
                        println!("IO error");
                        assert!(true);
                    },
                    Error::LogWriterError => {
                        println!("Log writer error");
                        panic!()
                    },
                    Error::Infallible => {
                        println!("Infallible");
                        panic!()
                    },
                    Error::IPError => {
                        println!("Ip Error");
                        panic!()
                    },
                    Error::NoPathSuppliedToLog => {
                        println!("No path supplied");
                        panic!()
                    },
                    Error::WhatTheFunkError => {
                        println!("What the funk error");
                        panic!()
                    }
                }
            },
        }
    }

}

