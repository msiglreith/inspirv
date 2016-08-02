
use std::result::Result;
use std::io::{Cursor, Read};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use num_traits::FromPrimitive;

use core::SPIRV_MAGIC;
use instruction::{Instruction, RawInstruction};
use io::ReadError;
use module::{Generator, GeneratorId, Header};
use types::*;

enum Endian {
    Little,
    Big,
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

    // TODO: merge peek&advance to consume? depends on optional data!
    pub fn peek(&mut self) -> Option<u32> {
        if !self.has_words() {
            None
        } else {
            let word = self.data.operands[self.operand_offset];
            Some(word)
        }
    }

    pub fn advance(&mut self) {
        self.operand_offset += 1;
    }

    pub fn has_words(&self) -> bool {
        self.operand_offset < self.data.operands.len()
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

    fn read_instruction_raw(&mut self) -> Result<RawInstruction, ReadError> {
        let first = try!(self.read_u32());
        let opcode = first & 0xFFFF;
        let num_words = ((first >> 16) & 0xFFFF).saturating_sub(1);
        let mut words = Vec::with_capacity(num_words as usize);

        for _ in 0 .. num_words {
            words.push(try!(self.read_u32()));
        }

        Ok(RawInstruction {
            opcode: opcode,
            operands: words,
        })
    }

    // TODO
    pub fn read_instruction(&mut self) -> Result<Instruction, ReadError> {
        let raw = try!(self.read_instruction_raw());
        Ok(Instruction::Unknown(raw))
    }
}

pub trait ReadBinaryExt: Sized {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError>;
}

impl ReadBinaryExt for u32 {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        if let Some(word) = view.peek() {
            view.advance();
            Ok(word)
        } else {
            Err(ReadError::OutOfOperands)
        }
    }
}

// TODO: high: untested!
impl ReadBinaryExt for String {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        let mut buf = Vec::new();
        'chars: while view.has_words() {
            let mut word = try!(u32::read(view));
            // handle utf8
            for _ in 0..4 {
                if (word & 0xFF) == 0 {
                    break 'chars;
                }

                buf.push((word & 0xFF) as u8);
                word >>= 8;
            }
        }

        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}

impl ReadBinaryExt for Id {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        Ok(Id(try!(u32::read(view))))
    }
}

impl ReadBinaryExt for LiteralInteger {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        Ok(LiteralInteger(try!(u32::read(view))))
    }
}

impl ReadBinaryExt for LiteralString {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        Ok(LiteralString(try!(String::read(view))))
    }
}

impl<U: ReadBinaryExt, V: ReadBinaryExt> ReadBinaryExt for (U, V) {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        Ok((try!(U::read(view)), try!(V::read(view))))
    }
}

// TODO: high: we currently consume the data, might fail?
impl<T: ReadBinaryExt> ReadBinaryExt for Option<T> {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        match T::read(view) {
            Ok(x) => Ok(Some(x)),
            _ => Ok(None),
        }
    }
}

impl<T: ReadBinaryExt> ReadBinaryExt for Vec<T> {
    fn read(view: &mut RawInstructionView) -> Result<Self, ReadError> {
        let mut data = Vec::new();
        while view.has_words() {
            if let Some(val) = try!(Option::<T>::read(view)) {
                data.push(val);
            } else {
                break;
            }
        }

        Ok(data)
    }
}

