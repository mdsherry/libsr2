use super::{simple_obj, Obj};
use crate::objects::{InGameTime, ItemId, PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRResNode {
    name: String,
    count1: i32,
    ts: InGameTime,
    resource_type: String,
    count2: i32,
    contents: Vec<ItemId>,
}
simple_obj!(
    SRResNode,
    "SRRESNODE",
    name,
    count1,
    ts,
    resource_type,
    count2,
    contents
);

impl PPrintable for SRResNode {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Name").value(&self.name);
            p.field("Resource type").value(&self.resource_type);
            p.field("Contents").value(&self.contents);
            p.ufield("Count 1").value(self.count1);
            p.ufield("Count 2").value(self.count2);
            p.ufield("Bytes").value(&self.ts);
        })
    }
}
