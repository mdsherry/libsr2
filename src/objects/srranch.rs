use crate::{primitives::{VecMap, InGameTime}, SRLP};

use super::simple_obj;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRANCH {
    pub plots: Vec<SRLP>,
    pub doors: VecMap<String, i32>,
    pub map2: VecMap<i32, i32>,
    pub map3: VecMap<String, InGameTime>,
}
simple_obj!(SRRANCH : 2, "SRRANCH", plots, doors, map2, map3);
