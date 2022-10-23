use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer, VecMap};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srpu {
    pub map: VecMap<i32, i32>,
}
impl PPrintable for &Srpu {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRPU", |p| {
            p.field("Purchased upgrades").map(&self.map.0, |p, (k, v)| {
                p.print(&format!("{}: {}", k, v));
            });
        })
    }
}
simple_obj!(Srpu, "SRPU", map);
