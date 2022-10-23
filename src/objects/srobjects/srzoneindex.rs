use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRZoneIndex {
    pub zones: Vec<String>,
}
simple_obj!(SRZoneIndex, "SRGZONEINDEX", zones);

impl PPrintable for SRZoneIndex {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("zones").value(&self.zones);
        })
    }
}
