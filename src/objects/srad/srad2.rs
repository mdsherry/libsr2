use crate::{Srv3, SRSed, primitives::{ItemId, InGameTime, SceneGroupId}, SRRCD, SRSE, simple_obj};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRAD2 {
    pub pos: Srv3,
    pub facing: Srv3,
    pub index: i32,
    pub unknown: [u8; 4],
    pub actor_type: ItemId,
    pub srsed: SRSed,
    // NaN for grown fowl, timestamp for chicks, 0 otherwise
    pub chick_timer: InGameTime,
    // NaN for chicks and roosters, timestamp for hens, 0 otherwise
    pub hen_timer: InGameTime,
    // Future time for plorts, 0 otherwise
    pub plort_expire: InGameTime,
    pub srrcd: SRRCD,
    pub ts4: Option<InGameTime>,
    pub feral: bool,
    pub more_bytes: [u8; 5],
    pub zone: SceneGroupId,
    pub statuified: bool,
    pub statue_actor_id: i32,
    pub even_more_bytes: [u8; 4],
    pub ts5: InGameTime,
    pub srses: Vec<SRSE>,
}
simple_obj!(SRAD2, "SRAD", pos, facing, index, unknown, actor_type, srsed, chick_timer, hen_timer, plort_expire, srrcd, ts4, feral, more_bytes,
zone, statuified, statue_actor_id, even_more_bytes, ts5, srses);