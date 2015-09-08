use bincode::rustc_serialize::DecodingError;

use std::net::TcpStream;
use std::io::{Read, Error};

use commands::ServerCommand;
use ser_de::de;

pub struct Reader {
    de_buf: Vec<u8>,
    buf: [u8; 4098],
}

impl Reader {
    
    pub fn new() -> Reader {
        Reader {
            de_buf: Vec::new(),
            buf: [0; 4098],
        }
    }
    
    pub fn read(&mut self, stream: &mut TcpStream) -> Result<(), Error> {
        let read = try!( stream.read(&mut self.buf[..]) );
        self.de_buf.push_all(&self.buf[0..read]);
        Ok(())
    }
    
    pub fn commands(&mut self) -> Result<Vec<ServerCommand>, DecodingError> {
        let mut res = Vec::new();
        
        loop {
            match try!( de(&mut self.de_buf) ) {
                Some(comm) => {
                    trace!("Incomig: {:?}", &comm);
                    res.push(comm)
                },
                None => { break; },
            }
        }
        
        Ok(res)
    }
    
}
