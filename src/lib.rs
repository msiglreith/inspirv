
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
pub mod write_binary;
pub mod io;