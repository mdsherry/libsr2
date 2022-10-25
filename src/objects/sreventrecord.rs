#[cfg(test)]
mod test;

use crate::{simple_obj, SREVENTENTRY};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SREVENTRECORD {
    pub events: Vec<SREVENTENTRY>,
}
simple_obj!(SREVENTRECORD, "SREVENTRECORD", events);
