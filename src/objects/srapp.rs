use super::simple_obj;
use crate::primitives::{ItemId, VecMap};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRAPP {
    pub item_map: VecMap<ItemId, (i32, i32)>,
    pub unknown: VecMap<ItemId, i32>,
}
simple_obj!(SRAPP, "SRAPP", item_map, unknown);
