
use core::instruction as core;
use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct RawInstruction {
    pub opcode: u32,
    pub operands: Vec<u32>,
}

#[derive(Debug)]
pub enum Instruction {
    Core(core::Instruction),
    Unknown(RawInstruction),
}

#[derive(Debug)]
pub enum OpCode {
    Core(core::OpCode),
    Unknown(u32),
}

pub trait InstructionExt {
    type OpCodeType;
    fn opcode(&self) -> Self::OpCodeType;
}

impl InstructionExt for RawInstruction {
    type OpCodeType = u32;
    fn opcode(&self) -> Self::OpCodeType {
        self.opcode
    }
}

impl InstructionExt for Instruction {
    type OpCodeType = OpCode;
    fn opcode(&self) -> Self::OpCodeType {
        match self {
            &Instruction::Core(ref instr) => OpCode::Core(instr.opcode()),
            &Instruction::Unknown(ref instr) => OpCode::Unknown(instr.opcode()),
        }
    }
}

impl OpCode {
    pub fn from_u32(n: u32) -> OpCode {
        if let Some(opcode) = core::OpCode::from_u32(n) {
            OpCode::Core(opcode)
        } else {
            OpCode::Unknown(n)
        }
    }
}
