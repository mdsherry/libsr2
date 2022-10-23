use crate::primitives::InGameTime;
use super::simple_obj;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRCD {
    pub condition: i32,
    pub time: InGameTime,
}
simple_obj!(SRRCD, "SRRCD", condition, time);
