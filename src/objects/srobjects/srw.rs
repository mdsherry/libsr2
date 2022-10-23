use super::{simple_obj, Obj, SRPg, SRReDrone, SRResNode, SRTp, Srrw, Srv3, SRG};
use crate::objects::{InGameTime, ItemId, PPrintable, Parseable, Printer, VecMap};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Srw {
    pub world_time: InGameTime,
    pub a: i32,
    pub b: f64,
    pub c: f64,
    pub d: InGameTime,
    pub e: i32,
    pub plort_market_saturation: VecMap<ItemId, f32>,
    pub hen_spawner_timers: VecMap<Srv3, InGameTime>,
    pub liquid_sources: VecMap<String, i32>,
    pub slime_spawner_timers: VecMap<Srv3, InGameTime>,
    pub gordos: VecMap<String, SRG>,
    pub map5: VecMap<Srv3, Srrw>,
    pub gadgets: VecMap<String, SRPg>,
    pub pods: VecMap<String, SRTp>,
    pub switches: VecMap<String, i32>,
    pub puzzles: VecMap<String, bool>,
    pub unknown2: i32, // Empty map?
    pub research_drones: VecMap<String, SRReDrone>,
    pub unknown3: i32, // Empty map?
    pub resource_nodes: VecMap<String, SRResNode>,
}
simple_obj!(
    Srw,
    "SRW",
    world_time,
    a, b, c, d, e,
    plort_market_saturation,
    hen_spawner_timers,
    liquid_sources,
    slime_spawner_timers,
    gordos,
    map5,
    gadgets,
    pods,
    switches,
    puzzles,
    unknown2,
    research_drones,
    unknown3,
    resource_nodes
);

impl PPrintable for Srw {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("World time").value(self.world_time);
            p.field("Plort market saturation")
                .value(&self.plort_market_saturation);
            p.field("Liquid sources").value(&self.liquid_sources);
            p.field("Gordos").value(&self.gordos);
            p.field("Gadgets").value(&self.gadgets);
            p.field("Pods").value(&self.pods);
            p.field("Switches").value(&self.switches);
            p.field("Puzzles").value(&self.puzzles);
            p.field("Research drones").value(&self.research_drones);
            p.field("Resource nodes").value(&self.resource_nodes);

            
            p.ufield("a").value(&self.a);
            p.ufield("b").value(&self.b);
            p.ufield("c").value(&self.c);
            p.ufield("d").value(&self.d);
            p.ufield("e").value(&self.e);
            

            p.ufield("Hen spawner timers").value(&self.hen_spawner_timers);
            p.ufield("Slime spawner timers").value(&self.slime_spawner_timers);
            p.ufield("map5").value(&self.map5);
            p.ufield("unknown2").value(&self.unknown2);
            p.ufield("unknown3").value(&self.unknown3);
        })
    }
}
