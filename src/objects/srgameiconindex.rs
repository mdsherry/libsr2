#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRGAMEICONINDEX {
    pub game_icons: Vec<String>,
}
simple_obj!(SRGAMEICONINDEX, "SRGAMEICONINDEX", game_icons);
