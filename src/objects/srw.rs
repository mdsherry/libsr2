use crate::primitives::{InGameTime, VecMap, ItemId};

use super::{simple_obj, SRPG, SRREDRONE, SRResNode, SRTp, Srrw, Srv3, SRG};

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
    pub gadgets: VecMap<String, SRPG>,
    pub pods: VecMap<String, SRTp>,
    pub switches: VecMap<String, i32>,
    pub puzzles: VecMap<String, bool>,
    pub unknown2: i32, // Empty map?
    pub research_drones: VecMap<String, SRREDRONE>,
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
