use super::Obj;
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Clone, Default, PartialEq)]
pub struct Srv3 {
    pub coords: [f32; 3],
}

impl Obj for Srv3 {
    const NAME: &'static str = "SRV3";
    fn parse_body(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, a) = f32::parse(input)?;
        let (input, b) = f32::parse(input)?;
        let (input, c) = f32::parse(input)?;
        Ok((input, Self { coords: [a, b, c] }))
    }

    fn write_body<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.coords[0].write(f)?;
        self.coords[1].write(f)?;
        self.coords[2].write(f)?;
        Ok(())
    }
}

impl Debug for Srv3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "V3({}, {}, {})",
            self.coords[0], self.coords[1], self.coords[2]
        )
    }
}

impl PPrintable for Srv3 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self:?}"));
    }
}
