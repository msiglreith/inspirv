
use core::instruction as core_instruction;
use num_traits::FromPrimitive;

// TODO: low: we maybe want to remove this in the future, depends if it's use later on
pub trait InstructionExt {
    type OpCodeType;
    fn opcode(&self) -> Self::OpCodeType;
}

#[derive(Clone, Debug)]
pub struct RawInstruction {
    pub opcode: u32,
    pub operands: Vec<u32>,
}

impl InstructionExt for RawInstruction {
    type OpCodeType = u32;
    fn opcode(&self) -> Self::OpCodeType {
        self.opcode
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Core(core_instruction::Instruction),
    Unknown(RawInstruction),
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

#[derive(Debug)]
pub enum OpCode {
    Core(core_instruction::OpCode),
    Unknown(u32),
}

impl OpCode {
    pub fn from_u32(n: u32) -> OpCode {
        if let Some(opcode) = core_instruction::OpCode::from_u32(n) {
            OpCode::Core(opcode)
        } else {
            OpCode::Unknown(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum BranchInstruction {
    Branch(core_instruction::OpBranch),
    BranchConditional(core_instruction::OpBranchConditional),
    Switch(core_instruction::OpSwitch),
    Kill(core_instruction::OpKill),
    Return(core_instruction::OpReturn),
    ReturnValue(core_instruction::OpReturnValue),
    Unreachable(core_instruction::OpUnreachable),
}

impl From<BranchInstruction> for Instruction {
    fn from(branch: BranchInstruction) -> Self {
        match branch {
            BranchInstruction::Branch(instr) => Instruction::Core(
                core_instruction::Instruction::OpBranch(instr)
            ),
            BranchInstruction::BranchConditional(instr) => Instruction::Core(
                core_instruction::Instruction::OpBranchConditional(instr)
            ),
            BranchInstruction::Switch(instr) => Instruction::Core(
                core_instruction::Instruction::OpSwitch(instr)
            ),
            BranchInstruction::Kill(instr) => Instruction::Core(
                core_instruction::Instruction::OpKill(instr)
            ),
            BranchInstruction::Return(instr) => Instruction::Core(
                core_instruction::Instruction::OpReturn(instr)
            ),
            BranchInstruction::ReturnValue(instr) => Instruction::Core(
                core_instruction::Instruction::OpReturnValue(instr)
            ),
            BranchInstruction::Unreachable(instr) => Instruction::Core(
                core_instruction::Instruction::OpUnreachable(instr)
            ),
        }
    }
}
