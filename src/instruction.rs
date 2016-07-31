
pub trait Instruction {
    type OpCodeType;
    fn opcode() -> Self::OpCodeType;
}
