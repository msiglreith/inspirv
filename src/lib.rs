
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate num_traits;

pub mod module;
pub mod core;
pub mod types;
pub mod instruction;
pub mod read_binary;

use std::io;

// https://doc.rust-lang.org/book/error-handling.html#composing-custom-error-types
#[derive(Debug)]
pub enum ReadError {
    Io(io::Error),
    InvalidSpirvMagic(u32),
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::Io(err)
    }
}

