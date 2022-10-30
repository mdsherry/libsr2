#[cfg(test)]
mod test;

use crate::primitives::{InGameTime, ItemId, VecMap};

use super::{simple_obj, SRRESNODE, SRTP, SRRW, SRV3, SRG, SRPG, SRREDRONE};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRW {
    pub world_time: InGameTime,
    pub a: i32,
    pub b: f64,
    pub c: f64,
    pub d: InGameTime,
    pub e: i32,
    pub plort_market_saturation: VecMap<ItemId, f32>,
    pub hen_spawner_timers: VecMap<SRV3, InGameTime>,
    // To differentiate regular water from special sparkly water in the desert?
    pub liquid_sources: VecMap<String, f32>,
    pub slime_spawner_timers: VecMap<SRV3, InGameTime>,
    pub gordos: VecMap<String, SRG>,
    pub map5: VecMap<SRV3, SRRW>,
    pub gadgets: VecMap<String, SRPG>,
    pub pods: VecMap<String, SRTP>,
    pub switches: VecMap<String, i32>,
    pub puzzles: VecMap<String, bool>,
    pub unknown2: i32, // Empty map?
    pub research_drones: VecMap<String, SRREDRONE>,
    pub unknown3: i32, // Empty map?
    pub resource_nodes: VecMap<String, SRRESNODE>,
}
simple_obj!(
    SRW,
    "SRW",
    world_time,
    a,
    b,
    c,
    d,
    e,
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
