#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRPED {
    pub a: i32, // Probably an array?
    pub index: Vec<String>,
}
simple_obj!(SRPED, "SRPED", a, index);
