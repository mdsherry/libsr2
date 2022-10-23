use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srsgi {
    pub scene_groups: Vec<String>,
}
simple_obj!(Srsgi, "SRSGI", scene_groups);

impl PPrintable for Srsgi {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Scene groups").value(&self.scene_groups);
        })
    }
}
