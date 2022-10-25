#[cfg(test)]
mod test;

use super::simple_obj;
use crate::primitives::VecMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRPU {
    pub map: VecMap<i32, i32>,
}
simple_obj!(SRPU, "SRPU", map);
