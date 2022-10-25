#[cfg(test)]
mod test;

use crate::primitives::InGameTime;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRW {
    pub ts: InGameTime,
    pub c: i32,
}

simple_obj!(SRRW, "SRRW", ts, c);
