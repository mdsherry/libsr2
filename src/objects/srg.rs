use crate::primitives::ItemId;

use super::{simple_obj, Srv3};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SRG {
    // Gordos
    pub fed: i32,
    pub unknown: i32,
    pub found: bool,
    pub pos: Srv3,
    pub facing: Srv3,
    pub gordo_type: ItemId,
}
simple_obj!(SRG, "SRG", fed, unknown, found, pos, facing, gordo_type);
