use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRUCI {
    pub upgrade_components: Vec<String>,
}
simple_obj!(SRUCI, "SRUCI", upgrade_components);

impl PPrintable for SRUCI {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Upgrade components")
                .value(&self.upgrade_components);
        })
    }
}
