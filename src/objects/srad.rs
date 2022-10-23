use crate::parsers::Parseable;

use nom::IResult;
use std::{fmt::Debug, io::Write};
mod srad1;
mod srad2;
pub use srad1::SRAD1;
pub use srad2::SRAD2;

#[derive(Debug, Clone, PartialEq)]
pub enum SRAD {
    V1(SRAD1),
    V2(SRAD2),
}
impl SRAD {
    pub fn as_v1(&self) -> Option<&SRAD1> {
        match self {
            SRAD::V1(v1) => Some(v1),
            SRAD::V2(_) => None,
        }
    }
    pub fn as_v2(&self) -> Option<&SRAD2> {
        match self {
            SRAD::V1(_) => None,
            SRAD::V2(v2) => Some(v2),
        }
    }
}
impl Default for SRAD {
    fn default() -> Self {
        SRAD::V1(Default::default())
    }
}
impl Parseable for SRAD {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        nom::branch::alt((
            nom::combinator::map(SRAD1::parse, SRAD::V1),
            nom::combinator::map(SRAD2::parse, SRAD::V2),
        ))(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        match self {
            SRAD::V1(v1) => v1.write(f),
            SRAD::V2(v2) => v2.write(f),
        }
    }
}
