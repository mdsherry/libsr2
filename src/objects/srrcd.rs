#[cfg(test)]
mod test;

use super::simple_obj;
use crate::primitives::InGameTime;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRCD {
    pub condition: i32,
    pub time: InGameTime,
}
simple_obj!(SRRCD, "SRRCD", condition, time);
