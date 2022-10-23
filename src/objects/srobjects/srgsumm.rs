use super::{simple_obj, Obj};
use crate::objects::{InGameTime, PPrintable, Parseable, Printer, TimeSinceYear1};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRGsumm {
    pub build: String,
    pub money: i32,
    pub discoveries: i32,
    pub time: InGameTime,
    pub save_datetime: TimeSinceYear1,
    pub unknown: [u8; 23],
    pub game_icon: i32,
}
simple_obj!(
    SRGsumm,
    "SRGSUMM",
    build,
    money,
    discoveries,
    time,
    save_datetime,
    unknown,
    game_icon
);

impl PPrintable for SRGsumm {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRGSUMM", |p| {
            p.field("Build").value(&self.build);
            p.field("Time").value(&self.time);
            p.field("Save time").value(&self.save_datetime);
            p.field("Money").value(&self.money);
            p.field("Discoveries").value(&self.discoveries);
            p.field("Game icon").value(&self.game_icon);
            p.ufield("unknown").value(&self.unknown);
        })
    }
}
