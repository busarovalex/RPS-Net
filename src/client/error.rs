use std::io::Error;
use bincode::rustc_serialize::DecodingError;
use std::convert::From;

#[derive(Debug)]
pub enum IterError {
    IO(Error),
    Decode(DecodingError),
}

impl From<Error> for IterError {
    fn from(e: Error) -> Self {
        IterError::IO(e)
    }
}

impl From<DecodingError> for IterError {
    fn from(e: DecodingError) -> Self {
        IterError::Decode(e)
    }
}
