
// TODO: These types are only temporary set, might change in the future

/// A numerical ID to refer to an object, type, function, label, ...
#[derive(Debug)]
pub struct Id(pub u32);

pub type IdResult = Id;
pub type IdResultType = Id;
pub type IdRefType = Id;
pub type IdRef = Id;
pub type IdScope = Id;
pub type IdMemorySemantics = Id;

pub struct Literal(pub String);

pub type LiteralInteger = Literal;
pub type LiteralExtInstInteger = Literal;
pub type LiteralString = Literal;
pub type LiteralContextDependentNumber = Literal;
pub type LiteralSpecConstantOpInteger = Literal;

pub struct PairIdRefIdRef(IdRef, IdRef);
pub struct PairIdRefLiteralInteger(IdRef, LiteralInteger);
pub struct PairLiteralIntegerIdRef(LiteralInteger, IdRef);
