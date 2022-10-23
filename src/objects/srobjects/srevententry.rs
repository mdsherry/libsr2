use super::{simple_obj, Obj};
use crate::objects::{InGameTime, PPrintable, Parseable, Printer, WindowsTime};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SREventEntry {
    pub typ: String,
    pub info: String,
    pub count: i32,
    pub first_updated: InGameTime,
    pub last_updated: InGameTime,
    pub first_updated_walltime: WindowsTime,
    pub last_updated_walltime: WindowsTime,
}
simple_obj!(
    SREventEntry,
    "SREVENTENTRY",
    typ,
    info,
    count,
    first_updated,
    last_updated,
    first_updated_walltime,
    last_updated_walltime
);

impl PPrintable for SREventEntry {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Type").value(&self.typ);
            p.field("Info").value(&self.info);
            p.field("Count").value(self.count);
            p.field("First updated").value(&self.first_updated);
            p.field("Last updated").value(self.last_updated);
            p.field("First updated (wall time)")
                .value(&self.first_updated_walltime);
            p.field("Last updated (wall time)")
                .value(&self.last_updated_walltime);
        })
    }
}
