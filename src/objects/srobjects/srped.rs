use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srped {
    pub a: i32, // Probably an array?
    pub index: Vec<String>,
}
simple_obj!(Srped, "SRPED", a, index);

impl PPrintable for Srped {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("a").value(self.a);
            p.field("Index").value(&self.index);
        })
    }
}
