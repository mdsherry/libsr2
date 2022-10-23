use super::{simple_obj, Obj, SRAD, TrackedActorList};
use crate::objects::{InGameTime, PPrintable, Parseable, PlantId, Printer, VecMap};
use nom::IResult;
use std::{fmt::Debug, io::Write};

// Plot?
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRLp {
    pub ran_feeder: InGameTime,
    pub unknown: [u8; 8],
    pub ran_vaccuum: InGameTime,
    pub unknown2: [u8; 8],
    pub plot_type: i32,
    pub plant_type: PlantId,
    pub name: String,
    pub numbers: Vec<i32>,
    pub plot_inventories: VecMap<String, Vec<SRAD>>,
    pub upgrades: Vec<i32>,

    pub f: f32,
    pub b1: bool,
    pub tracked_actor_list: TrackedActorList,
}
simple_obj!(SRLp : 2, "SRLP", ran_feeder, unknown, ran_vaccuum, unknown2, plot_type, plant_type, name, numbers, plot_inventories, upgrades, f, b1, tracked_actor_list);
impl PPrintable for SRLp {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRLP", |p| {
            p.field("Plot type").value(self.plot_type);
            p.field("Plant type").value(self.plant_type);
            p.field("Name").value(&self.name);
            p.field("Plot inventories").value(&self.plot_inventories);
            p.field("Upgrades").value(&self.upgrades);
            p.field("Tracked actor list")
                .value(&self.tracked_actor_list);

            p.ufield("Ran feeder").value(&self.ran_feeder);
            p.ufield("Ran vaccuum").value(&self.ran_vaccuum);

            p.ufield("Numbers").value(&self.numbers);
            p.ufield("Unknown").value(&self.unknown);
            p.ufield("Unknown2").value(&self.unknown2);
            p.ufield("f").value(self.f);
            p.ufield("b1").value(self.b1);
        })
    }
}
