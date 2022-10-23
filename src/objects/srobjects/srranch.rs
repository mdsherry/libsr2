use super::{simple_obj, Obj, SRLp};
use crate::objects::{InGameTime, PPrintable, Parseable, Printer, VecMap};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srranch {
    pub plots: Vec<SRLp>,
    pub doors: VecMap<String, i32>,
    pub map2: VecMap<i32, i32>,
    pub map3: VecMap<String, InGameTime>,
}
simple_obj!(Srranch : 2, "SRRANCH", plots, doors, map2, map3);
impl PPrintable for Srranch {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRRANCH", |p| {
            p.field("Plots").value(&self.plots);
            p.ufield("Doors").value(&self.doors);
            p.ufield("map2").value(&self.map2);
            p.ufield("map3").value(&self.map3);
        })
    }
}
