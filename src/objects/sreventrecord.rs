use crate::{simple_obj, SREventEntry};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SREventRecord {
    pub events: Vec<SREventEntry>,
}
simple_obj!(SREventRecord, "SREVENTRECORD", events);
