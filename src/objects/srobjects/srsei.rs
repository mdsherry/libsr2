use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSei {
    hashes: Vec<String>,
}
simple_obj!(SRSei, "SRSEI", hashes);

impl PPrintable for SRSei {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("hashes").value(&self.hashes);
        })
    }
}
