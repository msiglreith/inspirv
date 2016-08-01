
use core::instruction as core;

pub struct RawInstruction {
    pub opcode: u32,
    pub operands: Vec<u32>,
}

pub enum Instruction {
    Core(core::Instruction),
    Unkown(RawInstruction),
}

pub enum OpCode {
    Core(core::OpCode),
    Unkown(u32),
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
            &Instruction::Unkown(ref instr) => OpCode::Unkown(instr.opcode()),
        }
    }
}
