#[cfg(test)]
mod test;

use crate::primitives::{InGameTime, PlantId, VecMap};

use super::{simple_obj, TRACKEDACTORLIST, SRAD};

// Plot?
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRLP {
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
    pub tracked_actor_list: TRACKEDACTORLIST,
}
simple_obj!(SRLP : 2, "SRLP", ran_feeder, unknown, ran_vaccuum, unknown2, plot_type, plant_type, name, numbers, plot_inventories, upgrades, f, b1, tracked_actor_list);
