#[cfg(test)]
mod test;

use crate::{
    primitives::{InGameTime, WindowsTime},
    simple_obj,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SREVENTENTRY {
    pub typ: String,
    pub info: String,
    pub count: i32,
    pub first_updated: InGameTime,
    pub last_updated: InGameTime,
    pub first_updated_walltime: WindowsTime,
    pub last_updated_walltime: WindowsTime,
}
simple_obj!(
    SREVENTENTRY,
    "SREVENTENTRY",
    typ,
    info,
    count,
    first_updated,
    last_updated,
    first_updated_walltime,
    last_updated_walltime
);
