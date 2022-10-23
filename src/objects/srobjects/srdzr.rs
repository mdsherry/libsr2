use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srdzr {
    pub unknown: [u8; 8],
}
simple_obj!(Srdzr, "SRDZR", unknown);

impl PPrintable for Srdzr {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown").value(&self.unknown);
        })
    }
}
