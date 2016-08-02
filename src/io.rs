
use std::io;

// https://doc.rust-lang.org/book/error-handling.html#composing-custom-error-types
#[derive(Debug)]
pub enum ReadError {
    Io(io::Error),
    InvalidSpirvMagic(u32),
    InvalidEnumValue(u32),
    OutOfOperands, // TODO: some better error message, containing actual information
    ExpectedWord,
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::Io(err)
    }
}

