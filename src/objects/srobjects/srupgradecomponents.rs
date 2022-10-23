use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRUpgradeComponents {
    pub bytes: [u8; 4],
}
simple_obj!(SRUpgradeComponents, "SRUPGRADECOMPONENTS", bytes);

impl PPrintable for SRUpgradeComponents {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRUPGRADECOMPONENTS", |p| {
            p.ufield("bytes").value(&self.bytes);
        });
    }
}
