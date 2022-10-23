use crate::{primitives::ItemId, SRSed, simple_obj};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRAD1 {
    pub item_id: ItemId,
    pub count: i32,
    pub c: i32,
    pub sed: SRSed,
}
simple_obj!(SRAD1, "SRAD", item_id, count, c, sed);
