use super::{simple_obj, Obj, Srv3};
use crate::objects::{ItemId, PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SRG {
    // Gordos
    pub fed: i32,
    pub unknown: i32,
    pub found: bool,
    pub pos: Srv3,
    pub facing: Srv3,
    pub gordo_type: ItemId,
}
simple_obj!(SRG, "SRG", fed, unknown, found, pos, facing, gordo_type);

impl PPrintable for SRG {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRG", |p| {
            p.field("Position").value(&self.pos);
            p.field("Facing").value(&self.facing);
            p.field("Fed").value(self.fed);
            p.field("Found").value(self.found);
            p.field("Gordo type").value(&self.gordo_type);
            p.ufield("unknown").value(&self.unknown);
        });
    }
}
