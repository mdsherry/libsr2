#[cfg(test)]
mod test;

use super::simple_obj;
use crate::primitives::{InGameTime, TimeSinceYear1};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRGSUMM {
    pub build: String,
    pub money: i32,
    pub discoveries: i32,
    pub time: InGameTime,
    pub save_datetime: TimeSinceYear1,
    pub unknown: [u8; 23],
    pub game_icon: i32,
}
simple_obj!(
    SRGSUMM,
    "SRGSUMM",
    build,
    money,
    discoveries,
    time,
    save_datetime,
    unknown,
    game_icon
);
