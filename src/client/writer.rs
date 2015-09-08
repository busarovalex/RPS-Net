use std::net::TcpStream;
use std::io::{Write, Error};

use commands::ClientCommand;
use ser_de::ser;

pub struct Writer {
    to_write: Vec<u8>,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            to_write: Vec::new(),
        }
    }
    
    pub fn write(&mut self, stream: &mut TcpStream) -> Result<(), Error> {
        let written = try!( stream.write(&self.to_write[..]) );
        trace!("Written, {}", written);
        let mut still = self.to_write.split_off(written);
        ::std::mem::swap(&mut self.to_write, &mut still);
        Ok(())
    }
    
    pub fn push(&mut self, command: ClientCommand) {
        self.to_write.push_all(&ser(command)[..]);
    }
}
