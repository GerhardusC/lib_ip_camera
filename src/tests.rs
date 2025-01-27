#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr, env};

    use crate::utils::log_buffer;
    use crate::camera_control::CameraControl;
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
        let dummy_ip = env::var("DUMMY_INACCESSIBLE_IP").expect("You should set your DUMMY_INACCESSIBLE_IP environment variable to run tests.");
        let mut control = CameraControl::new(&dummy_ip, 554);
        control.set_reconnect_timeout(1);
        control.set_reconnect_count(1);
        println!("{control:?}");
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
                        panic!();
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
                    },
                    Error::ConnectionError => {
                        println!("Connection error");
                        assert!(true);
                    }
                }
            },
        }
    }

    #[test]
    fn connects_to_camera() {
        let dummy_ip = env::var("DUMMY_IP").expect("You should set your DUMMY_IP environment variable to run tests.");
        let mut control = CameraControl::new(&dummy_ip, 554);
        control.set_reconnect_timeout(1);
        control.set_reconnect_count(1);
        println!("{control:?}");
        control.connect().expect("Should be able to connect to DUMMY_IP");
    }
}

