use std::{io::Write, net::TcpStream, path::PathBuf};

use crate::{error::Error, utils::log_buffer};
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct CameraControl {
    ip: String,
    stream: Option<TcpStream>,
    logging_enabled: bool,
    log_location:  Option<PathBuf>,
    port: u32, // u16 only goes to 65535
}

impl CameraControl {
    pub fn new(ip: String, port: u32) -> CameraControl {
        CameraControl {
            ip,
            port,
            log_location: None,
            logging_enabled: false,
            stream: None,
        }
    }
    pub fn connect (&mut self) -> Result<&Self, Error>{
        {
            self.stream = TcpStream::connect()
        }
    }

    pub fn enable_logging(&mut self, log_location: PathBuf) {
        self.logging_enabled = true;
        self.log_location = Some(PathBuf::from("./logs"));
    }
}

pub fn move_camera(stream: &mut TcpStream, control_opts: CameraControl) -> Result<(), Error> {
    let ip = stream.peer_addr()?;
    let direction_string = match control_opts.direction {
        Direction::UP => "UP",
        Direction::RIGHT => "RIGHT",
        Direction::DOWN => "DWON",
        Direction::LEFT => "LEFT",
    };

    let control = format!(
        "SET_PARAMETER RTSP/1.0\r
        Content-type: ptzCmd: {}\r
        CSeq: 2\r\n
        session:\n",
        direction_string
    );

    let bytes_written = stream.write(control.as_bytes())?;
    let message = format!(
        "Camera IP: {}\nBytes Written: {}\nDirection: {}\n",
        ip, bytes_written, direction_string
    );

    if control_opts.logging_enabled {
        match control_opts.log_location {
            Some(path) => log_buffer(path, message.as_bytes())?,
            None => println!("No path supplied to log to."),
        }
    }

    Ok(())
}

