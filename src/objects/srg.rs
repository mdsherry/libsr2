#[cfg(test)]
mod test;

use crate::primitives::ItemId;

use super::{simple_obj, SRV3};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SRG {
    // Gordos
    pub fed: i32,
    pub unknown: i32,
    pub found: bool,
    pub pos: SRV3,
    pub facing: SRV3,
    pub gordo_type: ItemId,
}
simple_obj!(SRG, "SRG", fed, unknown, found, pos, facing, gordo_type);
