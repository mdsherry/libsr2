use super::{simple_obj, Obj, SREventEntry};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SREventRecord {
    pub events: Vec<SREventEntry>,
}
simple_obj!(SREventRecord, "SREVENTRECORD", events);

impl PPrintable for SREventRecord {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("events").value(&self.events);
        })
    }
}
