use crate::primitives::{ItemId, InGameTime};

use super::simple_obj;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRResNode {
    pub name: String,
    pub count1: i32,
    pub ts: InGameTime,
    pub resource_type: String,
    pub count2: i32,
    pub contents: Vec<ItemId>,
}
simple_obj!(
    SRResNode,
    "SRRESNODE",
    name,
    count1,
    ts,
    resource_type,
    count2,
    contents
);
