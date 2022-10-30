#[cfg(test)]
mod test;

use crate::StringPair;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRGAMESETTINGS {
    pub game_settings: Vec<StringPair>,
    pub game_icon: i32,
}
simple_obj!(SRGAMESETTINGS, "SRGAMESETTINGS", game_settings, game_icon);
