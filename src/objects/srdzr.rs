use crate::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRDZR {
    pub unknown: [u8; 8],
}
simple_obj!(SRDZR, "SRDZR", unknown);