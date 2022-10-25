#[cfg(test)]
mod test;

use super::{
    simple_obj, SREVENTRECORD, SRGAMEICONINDEX, SRGAMESETTINGS, SRSECRETSTYLEDISC, SRGZONEINDEX,
    SRRGI, SRW, SRAD, SRAPP, SRGSUMM, SRITI, SRPED, SRPL, SRRANCH, SRSEI, SRSGI, SRUCI,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGAME {
    pub item_index: SRITI,
    pub patch_index: SRRGI,
    pub game_icon_index: SRGAMEICONINDEX,
    pub zone_index: SRGZONEINDEX,
    pub build_date: String,
    pub str1: String,
    pub bytes1: i32,
    pub save_game_summary: SRGSUMM,
    pub game_settings: SRGAMESETTINGS,
    pub scene_group_index: SRSGI,
    pub srw: SRW,
    pub player: SRPL,
    pub ranch: SRRANCH,
    pub unknown1: [u8; 2],
    pub srads: Vec<SRAD>,
    pub unknown2: [u8; 2],
    pub ped: SRPED,
    pub app: SRAPP,
    pub secret_style_discs: SRSECRETSTYLEDISC,
    pub event_record: SREVENTRECORD,
    pub uc_index: SRUCI,
    pub srse_index: SRSEI,
}

simple_obj!(
    SRGAME,
    "SRGAME",
    item_index,
    patch_index,
    game_icon_index,
    zone_index,
    build_date,
    str1,
    bytes1,
    save_game_summary,
    game_settings,
    scene_group_index,
    srw,
    player,
    ranch,
    unknown1,
    srads,
    unknown2,
    ped,
    app,
    secret_style_discs,
    event_record,
    uc_index,
    srse_index
);
