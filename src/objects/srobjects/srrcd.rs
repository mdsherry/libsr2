use super::{simple_obj, Obj};
use crate::objects::{InGameTime, PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srrcd {
    pub condition: i32,
    pub time: InGameTime,
}
simple_obj!(Srrcd, "SRRCD", condition, time);

impl PPrintable for Srrcd {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRRCD", |p| {
            p.field("Condition").value(self.condition);
            p.field("State transition time").value(self.time);
        })
    }
}
