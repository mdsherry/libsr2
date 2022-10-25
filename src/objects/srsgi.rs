#[cfg(test)]
mod test;

use super::simple_obj;
use std::fmt::Debug;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSGI {
    pub scene_groups: Vec<String>,
}
simple_obj!(SRSGI, "SRSGI", scene_groups);
