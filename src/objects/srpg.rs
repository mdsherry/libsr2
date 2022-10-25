#[cfg(test)]
mod test;

use crate::{
    primitives::{ItemId, SceneGroupId},
    SRV3,
};

use super::simple_obj;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SRPG {
    pub item_id: ItemId,
    pub actor_id: i32,
    pub c: i32,
    pub angle: f32,
    // Consider: could contain inventory if a warp depot, or pointer to other endpoint
    pub bytes: [u8; 32],
    pub scene_group_id: SceneGroupId,
    pub e: [u8; 5],
    pub f: SRV3,
}

simple_obj!(
    SRPG,
    "SRPG",
    item_id,
    actor_id,
    c,
    angle,
    bytes,
    scene_group_id,
    e,
    f
);
