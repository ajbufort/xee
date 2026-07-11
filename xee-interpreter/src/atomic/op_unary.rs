use crate::atomic;
use crate::error;

use super::cast_binary::cast_untyped_arithmetic;

pub(crate) fn unary_plus(atomic: atomic::Atomic) -> error::Result<atomic::Atomic> {
    // https://www.w3.org/TR/xpath-31/#id-arithmetic rule 4: an
    // untypedAtomic operand is cast to xs:double
    let atomic = cast_untyped_arithmetic(atomic)?;
    match &atomic {
        atomic::Atomic::Integer(_, _)
        | atomic::Atomic::Decimal(_)
        | atomic::Atomic::Float(_)
        | atomic::Atomic::Double(_) => Ok(atomic.clone()),
        _ => Err(error::Error::XPTY0004),
    }
}

pub(crate) fn unary_minus(atomic: atomic::Atomic) -> error::Result<atomic::Atomic> {
    let atomic = cast_untyped_arithmetic(atomic)?;
    match atomic {
        atomic::Atomic::Integer(_, i) => Ok((-i.as_ref().clone()).into()),
        atomic::Atomic::Decimal(d) => Ok((-*d.as_ref()).into()),
        atomic::Atomic::Float(f) => Ok((-f).into()),
        atomic::Atomic::Double(d) => Ok((-d).into()),
        _ => Err(error::Error::XPTY0004),
    }
}
