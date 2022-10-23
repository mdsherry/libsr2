use super::{simple_obj, Obj};
use crate::objects::{InGameTime, PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSe {
    pub unknown: [u8; 4],
    pub ts: InGameTime,
}
simple_obj!(SRSe, "SRSE", unknown, ts);

impl PPrintable for SRSe {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown").value(&self.unknown);
            p.ufield("ts").value(&self.ts);
        })
    }
}
