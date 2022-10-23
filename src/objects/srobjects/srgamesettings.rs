use super::{simple_obj, Obj, StringPair};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGameSettings {
    pub game_settings: Vec<StringPair>,
    pub game_icon: i32,
}
simple_obj!(SRGameSettings, "SRGAMESETTINGS", game_settings, game_icon);

impl PPrintable for SRGameSettings {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Game settings").value(&self.game_settings);
            p.field("Game icon").value(self.game_icon);
        });
    }
}
