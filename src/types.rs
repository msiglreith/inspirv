
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

pub struct LiteralString(pub String);
pub struct LiteralInteger(pub u32); // TODO: verify!

pub type LiteralExtInstInteger = LiteralInteger;
pub type LiteralContextDependentNumber = LiteralInteger;
pub type LiteralSpecConstantOpInteger = LiteralInteger;

pub type PairIdRefIdRef = (IdRef, IdRef);
pub type PairIdRefLiteralInteger = (IdRef, LiteralInteger);
pub type PairLiteralIntegerIdRef = (LiteralInteger, IdRef);
