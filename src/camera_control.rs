use std::{io::Write, net::{TcpStream, ToSocketAddrs}, path::PathBuf, thread::sleep, time::Duration};

use crate::{error::Error, utils::log_buffer};
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
pub struct CameraControl {
    ip: String,
    stream: Option<TcpStream>,
    logging_enabled: bool,
    log_location:  Option<PathBuf>,
    port: u32, // u16 only goes to 65535
    reconnect_timeout: u8,
    reconnect_count: u8,
}

impl CameraControl {
    pub fn new(ip: &str, port: u32) -> CameraControl {
        CameraControl {
            ip: ip.trim().to_owned(),
            port,
            log_location: None,
            logging_enabled: false,
            stream: None,
            reconnect_timeout: 1,
            reconnect_count: 5,       
        }
    }

    pub fn enable_logging(&mut self, log_location: PathBuf) -> &Self {
        self.logging_enabled = true;
        self.log_location = Some(log_location);
        self
    }

    pub fn set_reconnect_timeout(&mut self, seconds: u8) -> &Self {
        self.reconnect_timeout = seconds;
        self
    }

    pub fn set_reconnect_count(&mut self, count: u8) -> &Self {
        self.reconnect_count = count;
        self
    }
    pub fn connect (&mut self) -> Result<&mut Self, Error>{
        let ip_option = format!("{}:{}", self.ip, self.port).to_socket_addrs()?.last();
        let addr = match ip_option {
            Some(addr) => {
                addr
            },
            None =>{
                return Err(Error::IPError);
            },
        };
        
        match TcpStream::connect_timeout(&addr, Duration::from_secs(self.reconnect_timeout.into())) {
            Ok(stream) => {
                self.stream = Some(stream);
                return Ok(self);
            },
            Err(_) => {
                if self.reconnect_count > 0 {
                    println!("Reconnecting: {}.", self.reconnect_count);
                    self.reconnect_count -= 1;
                    sleep(Duration::from_secs(self.reconnect_timeout.into()));
                    return self.connect();
                } else {
                    return Err(Error::ConnectionError);
                }
            },
        }
    }

    pub fn move_camera(&mut self, direction: Direction) -> Result<(), Error> {
        let mut stream = match &self.stream {
            Some(stream) => {
                stream
            },
            None => {
                self.connect()?;
                return self.move_camera(direction);
            }
        };
        let ip = if let Some(stream) = &self.stream {
            stream.peer_addr()?
        } else {
            let ip_str = format!("{}:{}", self.ip, self.port);
            let ip = ip_str.to_socket_addrs()?.last();
            match ip {
                Some(addr) => addr,
                None => return Err(Error::IPError),
            }
        };
        let direction_string = match direction {
            Direction::UP => "UP",
            Direction::RIGHT => "RIGHT",
            // Yes, DWON, not DOWN, this is on the Yoosee camera, I don't know if its the same on any other cameras.
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

        if self.logging_enabled {
            match &self.log_location {
                Some(path) => log_buffer(path, message.as_bytes())?,
                None => return Err(Error::NoPathSuppliedToLog),
            }
        }

        Ok(())
    }
}


