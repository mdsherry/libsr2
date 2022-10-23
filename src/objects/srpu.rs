use crate::primitives::VecMap;
use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRPU {
    pub map: VecMap<i32, i32>,
}
simple_obj!(SRPU, "SRPU", map);
