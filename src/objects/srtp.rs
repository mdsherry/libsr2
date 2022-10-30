#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRTP {
    pub a: i32,
    pub b: i32,
}
simple_obj!(SRTP, "SRTP", a, b);
