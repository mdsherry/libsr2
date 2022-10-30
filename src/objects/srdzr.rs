#[cfg(test)]
mod test;

use crate::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRDZR {
    pub unknown: [u8; 8],
}
simple_obj!(SRDZR, "SRDZR", unknown);
