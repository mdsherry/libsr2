use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGameIconIndex {
    pub game_icons: Vec<String>,
}
simple_obj!(SRGameIconIndex, "SRGAMEICONINDEX", game_icons);
