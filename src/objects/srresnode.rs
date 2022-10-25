#[cfg(test)]
mod test;

use crate::primitives::{InGameTime, ItemId};

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRESNODE {
    pub name: String,
    pub count1: i32,
    pub ts: InGameTime,
    pub resource_type: String,
    pub count2: i32,
    pub contents: Vec<ItemId>,
}
simple_obj!(
    SRRESNODE,
    "SRRESNODE",
    name,
    count1,
    ts,
    resource_type,
    count2,
    contents
);
