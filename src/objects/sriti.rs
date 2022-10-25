#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRITI {
    pub identifiable_types: Vec<String>,
}
simple_obj!(SRITI, "SRITI", identifiable_types);
