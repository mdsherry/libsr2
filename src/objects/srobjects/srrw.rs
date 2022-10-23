use super::{simple_obj, Obj};
use crate::objects::{InGameTime, PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srrw {
    pub ts: InGameTime,
    pub c: i32,
}

simple_obj!(Srrw, "SRRW", ts, c);
impl PPrintable for Srrw {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRRW", |p| {
            p.ufield("ts").value(self.ts);
            p.ufield("c").value(self.c);
        })
    }
}
