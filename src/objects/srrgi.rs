#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRGI {
    pub patches: Vec<String>,
}
simple_obj!(SRRGI, "SRRGI", patches);
