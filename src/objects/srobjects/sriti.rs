use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Sriti {
    pub identifiable_types: Vec<String>,
}
simple_obj!(Sriti, "SRITI", identifiable_types);

impl PPrintable for Sriti {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRITI", |p| {
            p.field("identifiable_types")
                .value(&self.identifiable_types);
        });
    }
}
