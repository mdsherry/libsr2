use super::{simple_obj, Obj};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGameIconIndex {
    pub game_icons: Vec<String>,
}
simple_obj!(SRGameIconIndex, "SRGAMEICONINDEX", game_icons);

impl PPrintable for SRGameIconIndex {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Game icons").value(&self.game_icons);
        })
    }
}
