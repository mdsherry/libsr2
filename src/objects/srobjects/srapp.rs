use super::{simple_obj, Obj};
use crate::objects::{ItemId, PPrintable, Parseable, Printer, VecMap};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRApp {
    item_map: VecMap<ItemId, (i32, i32)>,
    unknown: VecMap<ItemId, i32>,
}
simple_obj!(SRApp, "SRAPP", item_map, unknown);

impl PPrintable for SRApp {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("item_map").value(&self.item_map);
            p.ufield("unknown").value(&self.unknown);
        })
    }
}
