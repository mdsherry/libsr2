use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]

pub struct Sed {
    pub unknown: Vec<f32>,
}

impl From<&SRSed> for Sed {
    fn from(s: &SRSed) -> Self {
        let mut unknown = vec![0.; s.unknown.len()];
        for (k, v) in &s.unknown {
            unknown[*k as usize] = *v;
        }
        Sed { unknown }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSed {
    pub unknown: Vec<(i32, f32)>,
}
simple_obj!(SRSed, "SRSED", unknown);
impl PPrintable for SRSed {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRSED", |p| {
            p.ufield("unknown").value(&self.unknown);
        });
    }
}
