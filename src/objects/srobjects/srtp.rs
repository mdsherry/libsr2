use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRTp {
    pub a: i32,
    pub b: i32,
}
simple_obj!(SRTp, "SRTP", a, b);

impl PPrintable for SRTp {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("a").value(self.a);
            p.ufield("b").value(self.b);
        })
    }
}
