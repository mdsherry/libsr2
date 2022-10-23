use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSecretStyleDisc {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
}
simple_obj!(SRSecretStyleDisc, "SRSECRETSTYLEDISC", a, b, c, d);

impl PPrintable for SRSecretStyleDisc {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("a").value(self.a);
            p.ufield("b").value(self.b);
            p.ufield("c").value(self.c);
            p.ufield("d").value(self.d);
        })
    }
}
