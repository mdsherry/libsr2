#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRUPGRADECOMPONENTS {
    pub bytes: [u8; 4],
}
simple_obj!(SRUPGRADECOMPONENTS, "SRUPGRADECOMPONENTS", bytes);
