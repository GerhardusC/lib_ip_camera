use std::{io::Write, net::TcpStream, path::PathBuf};

use crate::{error::Error, utils::log_buffer};
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub struct CameraControl {
    pub direction: Direction,
    pub logging_enabled: bool,
    pub log_location:  Option<PathBuf>,
}

impl CameraControl {
    pub fn new(direction: Direction) -> CameraControl {
        CameraControl {
            direction,
            log_location: None,
            logging_enabled: false,
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

