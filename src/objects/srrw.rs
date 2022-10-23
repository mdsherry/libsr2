use crate::primitives::InGameTime;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srrw {
    pub ts: InGameTime,
    pub c: i32,
}

simple_obj!(Srrw, "SRRW", ts, c);
