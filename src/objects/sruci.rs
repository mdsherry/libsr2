#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRUCI {
    pub upgrade_components: Vec<String>,
}
simple_obj!(SRUCI, "SRUCI", upgrade_components);
