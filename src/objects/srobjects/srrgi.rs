use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srrgi {
    pub patches: Vec<String>,
}
simple_obj!(Srrgi, "SRRGI", patches);

impl PPrintable for Srrgi {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRRGI", |p| {
            p.field("Patches").value(&self.patches);
        });
    }
}
