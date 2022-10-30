#[cfg(test)]
mod test;

use crate::parsers::Parseable;

use super::Obj;
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StringPair(pub String, pub String);

impl Obj for StringPair {
    const NAME: &'static str = "";
    fn parse_prefix(input: &[u8]) -> IResult<&[u8], (), nom::error::VerboseError<&[u8]>> {
        let (input, _) = nom::bytes::complete::tag([0])(input)?;
        let (input, _) = nom::bytes::complete::tag([0x00, 0x00, 0x00, 0x00, 0x00, 0x30])(input)?;
        Ok((input, ()))
    }
    fn parse_body(input: &[u8]) -> IResult<&[u8], Self, nom::error::VerboseError<&[u8]>> {
        let (input, (a, b)) = Parseable::parse(input)?;
        Ok((input, StringPair(a, b)))
    }
    fn write_prefix<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30])
    }
    fn write_body<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)?;
        self.1.write(f)?;
        Ok(())
    }
}
