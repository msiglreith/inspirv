
use std::result::Result;
use std::io::{Cursor, Read};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use num_traits::FromPrimitive;

use io::ReadError;
use module::{Generator, GeneratorId, Header};
use types::*;
use core::SPIRV_MAGIC;

enum Endian {
    Little,
    Big,
}

pub struct RawInstruction {
    pub opcode: u32,
    pub operands: Vec<u32>,
}

pub struct RawInstructionView<'a> {
    data: &'a RawInstruction,
    operand_offset: usize,
}

impl<'a> RawInstructionView<'a> {
    pub fn new(instruction: &'a RawInstruction) -> Self {
        RawInstructionView {
            data: instruction,
            operand_offset: 0,
        }
    }

    pub fn peek(&mut self) -> Option<u32> {
        if self.operand_offset >= self.data.operands.len() {
            None
        } else {
            let word = self.data.operands[self.operand_offset];
            Some(word)
        }
    }

    pub fn advance(&mut self) {
        self.operand_offset += 1;
    }
}

pub struct ReaderBinary<R: Read> {
    inner: R,
    endian: Endian,
    header: Header,
}

impl<R: Read> ReaderBinary<R> {
    pub fn new(inner: R) -> Result<Self, ReadError> {
        // set temporary values
        let mut reader = ReaderBinary {
            inner: inner,
            endian: Endian::Little,
            header: Header {
                magic_number: 0,
                version: (0, 0),
                generator: Generator::Unknown(0),
                bound: 0,
            },
        };

        // parse header
        // SPIR-V specs(1.1), Section 2.3
        let magic_number = try!(reader.read_u32());

        // set endian according to the magic number (Section 3.1)
        if magic_number == SPIRV_MAGIC {
            reader.endian = Endian::Little;
        } else if magic_number.swap_bytes() == SPIRV_MAGIC {
            reader.endian = Endian::Big;
        } else {
            return Err(ReadError::InvalidSpirvMagic(magic_number));
        }

        let version = {
            // 0 | Major Number | Minor Number | 0
            let raw = try!(reader.read_u32());
            let minor = (raw >> 8) & 0xFF;
            let major = (raw >> 16) & 0xFF;
            (major, minor)
        };

        let generator = {
            let id = try!(reader.read_u32());
            if let Some(gen_id) = GeneratorId::from_u32(id) {
                Generator::Id(gen_id)
            } else {
                Generator::Unknown(id)
            }
        };

        let bound = try!(reader.read_u32());

        reader.header = Header {
            magic_number: SPIRV_MAGIC,
            version: version,
            generator: generator,
            bound: bound,
        };

        Ok(reader)
    }

    fn read_u32(&mut self) -> Result<u32, ReadError> {
        let mut buf: [u8; 4] = [0; 4];
        try!(self.inner.read_exact(&mut buf));
        let mut rdr = Cursor::new(&mut buf);

        let word = match self.endian {
            Endian::Little => try!(rdr.read_u32::<LittleEndian>()),
            Endian::Big => try!(rdr.read_u32::<BigEndian>()),
        };

        Ok(word)
    }

    pub fn read_instruction_raw(&mut self) -> Result<RawInstruction, ReadError> {
        let first = try!(self.read_u32());
        let opcode = first & 0xFFFF;
        let mut words = {
            let num_words = (first >> 16) & 0xFFFF;
            Vec::with_capacity(num_words as usize - 1)
        };

        for i in 0 .. words.len() {
            words[i] = try!(self.read_u32());
        }

        Ok(RawInstruction {
            opcode: opcode,
            operands: words,
        })
    }
}

pub trait ReadBinaryExt: Sized {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError>;
}

impl ReadBinaryExt for Id {
    fn read(view: &mut RawInstructionView) -> Result<Id, ReadError> {
        if let Some(id) = view.peek() {
            view.advance();
            Ok(Id(id))
        } else {
            Err(ReadError::OutOfOperands)
        }
    }
}

impl<T: ReadBinaryExt> ReadBinaryExt for Option<T> {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        match T::read(view) {
            Ok(x) => Ok(Some(x)),
            _ => Ok(None),
        }
    }
}
