#[cfg(test)]
mod test;

use crate::{primitives::ItemId, simple_obj, SRSED};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRAD1 {
    pub item_id: ItemId,
    pub count: i32,
    pub c: i32,
    pub sed: SRSED,
}
simple_obj!(SRAD1, "SRAD", item_id, count, c, sed);
