use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRReDrone {
    discovered: bool,
    name: String,
}
simple_obj!(SRReDrone, "SRREDRONE", discovered, name);

impl PPrintable for SRReDrone {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Discovered").value(self.discovered);
            p.field("Name").value(&self.name);
        })
    }
}
