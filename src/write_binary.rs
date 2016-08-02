
use std::io::{Result, Write};
use module::{Generator, Header};
use instruction::{Instruction, RawInstruction};
use byteorder::{LittleEndian, ByteOrder};
use types::*;

pub struct WriterBinary<W> {
    inner: W,
}

impl<W: Write> WriterBinary<W> {
    pub fn new(inner: W) -> WriterBinary<W> {
        WriterBinary {
            inner: inner,
        }
    }

    fn write_u32(&mut self, word: u32) -> Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        LittleEndian::write_u32(&mut buf, word);
        self.inner.write_all(&buf)
    }

    pub fn write_header(&mut self, header: Header) -> Result<()> {
        try!(self.write_u32(header.magic_number));

        // 0 | Major Number | Minor Number | 0
        let version = ((header.version.0 & 0xFF) << 16) | (header.version.1 & 0xFF) << 8;
        try!(self.write_u32(version));

        match header.generator {
            Generator::Id(id) => try!(self.write_u32(id as u32)),
            Generator::Unknown(id) => try!(self.write_u32(id)),
        }

        try!(self.write_u32(header.bound));

        try!(self.write_u32(0)); // 0 (Reserved for instruction schema, if needed.)

        Ok(())
    }

    fn write_raw_instruction(&mut self, raw: &RawInstruction) -> Result<()> {
        let first = (raw.opcode & 0xFFFF) | (((raw.operands.len() as u32 + 1) & 0xFFFF) << 16);
        try!(self.write_u32(first));

        for operand in raw.operands.iter() {
            try!(self.write_u32(*operand));
        }

        Ok(())
    }

    pub fn write_instruction(&mut self, instr: &Instruction) -> Result<()> {
        match instr {
            &Instruction::Core(ref core) => self.write_raw_instruction(&core.to_raw()),
            &Instruction::Unknown(ref raw) => self.write_raw_instruction(raw),
        }
    }
}

pub trait WriteBinaryExt {
    fn write(&self, instr: &mut RawInstruction);
}

impl WriteBinaryExt for u32 {
    fn write(&self, instr: &mut RawInstruction) {
        instr.operands.push(*self);
    }
}

impl WriteBinaryExt for Id {
    fn write(&self, instr: &mut RawInstruction) {
        self.0.write(instr);
    }
}

impl WriteBinaryExt for LiteralInteger {
    fn write(&self, instr: &mut RawInstruction) {
        self.0.write(instr);
    }
}

impl WriteBinaryExt for LiteralString {
    fn write(&self, instr: &mut RawInstruction) {
        let mut chars = &mut self.0.chars();
        for _ in 0 .. (self.0.chars().count() / 4) + 1 {
            let mut word: u32 = 0;
            word |= chars.next().unwrap_or(0 as char) as u32 & 0xFF;
            word |= (chars.next().unwrap_or(0 as char) as u32 & 0xFF) << 8;
            word |= (chars.next().unwrap_or(0 as char) as u32 & 0xFF) << 16;
            word |= (chars.next().unwrap_or(0 as char) as u32 & 0xFF) << 24;
            word.write(instr);
        }
    }
}

impl<U: WriteBinaryExt, V: WriteBinaryExt> WriteBinaryExt for (U, V) {
    fn write(&self, instr: &mut RawInstruction) {
        self.0.write(instr);
        self.1.write(instr);
    }
}

impl<T: WriteBinaryExt> WriteBinaryExt for Option<T> {
    fn write(&self, instr: &mut RawInstruction) {
        match *self {
            Some(ref data) => data.write(instr),
            None => (),
        }
    }
}

impl<T: WriteBinaryExt> WriteBinaryExt for Vec<T> {
    fn write(&self, instr: &mut RawInstruction) {
        for data in self {
            data.write(instr);
        }
    }
}
