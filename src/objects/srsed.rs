#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSED {
    pub unknown: Vec<(i32, f32)>,
}
simple_obj!(SRSED, "SRSED", unknown);
