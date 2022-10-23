use crate::primitives::{ItemId, VecMap};

use super::{simple_obj, SRUpgradeComponents, SRAD, SRDZR, SRPU, Srv3};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRPL {
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
    pub srpu: SRPU,
    pub upgrade_components: SRUpgradeComponents,
    pub inventory: Vec<SRAD>,
    pub gadgets_unlocked: Vec<ItemId>,
    pub unknown: [u8; 4],
    pub refinery_contents: VecMap<ItemId, i32>,
    pub b1: bool,
    pub unknown2: [u8; 8],
    pub b2: bool,
    pub srdzr: SRDZR,
}
simple_obj!(
    SRPL,
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
