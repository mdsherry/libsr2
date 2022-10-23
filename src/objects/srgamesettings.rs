use crate::StringPair;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGameSettings {
    pub game_settings: Vec<StringPair>,
    pub game_icon: i32,
}
simple_obj!(SRGameSettings, "SRGAMESETTINGS", game_settings, game_icon);
