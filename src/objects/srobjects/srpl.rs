use super::{simple_obj, Obj, SRUpgradeComponents, SRAD, Srdzr, Srpu, Srv3};
use crate::objects::{ItemId, PPrintable, Parseable, Printer, VecMap};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRPl {
    // Player?
    pub hp: i32,
    pub energy: i32,
    pub a: i32,
    pub money: i32,
    pub c: i32,
    pub total_money_earned: i32,
    pub build: String,
    pub pos: Srv3,
    pub facing: Srv3,
    pub srpu: Srpu,
    pub upgrade_components: SRUpgradeComponents,
    pub inventory: Vec<SRAD>,
    pub gadgets_unlocked: Vec<ItemId>,
    pub unknown: [u8; 4],
    pub refinery_contents: VecMap<ItemId, i32>,
    pub b1: bool,
    pub unknown2: [u8; 8],
    pub b2: bool,
    pub srdzr: Srdzr,
}
impl PPrintable for SRPl {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRPL", |p| {
            p.field("HP").value(self.hp);
            p.field("Energy").value(self.energy);
            p.field("Money").value(self.money);
            p.field("Build").value(&self.build);
            p.field("Pos").value(&self.pos);
            p.field("Facing").value(&self.facing);
            p.field("Total money earned").value(self.total_money_earned);
            p.field("Purchased upgrades").value(&self.srpu);

            p.field("Gadgets unlocked").value(&self.gadgets_unlocked);

            p.field("Refinery contents").value(&self.refinery_contents);
            p.field("Upgrade components")
                .value(&self.upgrade_components);
            p.field("Inventory").value(&self.inventory);

            p.ufield("srdzr").value(&self.srdzr);
            p.ufield("a").value(self.a);
            p.ufield("c").value(self.c);
            p.ufield("b1").value(self.b1);
            p.ufield("b2").value(self.b2);
            p.ufield("unknown").value(&self.unknown);
            p.ufield("unknown").value(&self.unknown2);
        });
    }
}
simple_obj!(
    SRPl,
    "SRPL",
    hp,
    energy,
    a,
    money,
    c,
    total_money_earned,
    build,
    pos,
    facing,
    srpu,
    upgrade_components,
    inventory,
    gadgets_unlocked,
    unknown,
    refinery_contents,
    b1,
    unknown2,
    b2,
    srdzr
);
