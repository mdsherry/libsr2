#[cfg(test)]
mod test;

use crate::primitives::InGameTime;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSE {
    pub unknown: [u8; 4],
    pub ts: InGameTime,
}
simple_obj!(SRSE, "SRSE", unknown, ts);
