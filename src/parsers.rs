use std::io::Write;

use nom::IResult;

use crate::simple_parse;

mod primitives;

pub trait Parseable: Sized {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>;

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()>;
}

impl<A, B> Parseable for (A, B)
where
    A: Parseable,
    B: Parseable,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        simple_parse!(input, a, b);
        Ok((input, (a, b)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)?;
        self.1.write(f)
    }
}
