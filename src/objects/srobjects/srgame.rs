use super::{
    simple_obj, Obj, SRApp, SREventRecord, SRGameIconIndex, SRGameSettings, SRGsumm, SRPl,
    SRSecretStyleDisc, SRSei, SRZoneIndex, SRAD, Sriti, Srped, Srranch, Srrgi, Srsgi, Srw, SRUCI,
};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGame {
    pub item_index: Sriti,
    pub patch_index: Srrgi,
    pub game_icon_index: SRGameIconIndex,
    pub zone_index: SRZoneIndex,
    pub build_date: String,
    pub str1: String,
    pub bytes1: i32,
    pub save_game_summary: SRGsumm,
    pub game_settings: SRGameSettings,
    pub scene_group_index: Srsgi,
    pub srw: Srw,
    pub player: SRPl,
    pub ranch: Srranch,
    pub unknown1: [u8; 2],
    pub srads: Vec<SRAD>,
    pub unknown2: [u8; 2],
    pub ped: Srped,
    pub app: SRApp,
    pub secret_style_discs: SRSecretStyleDisc,
    pub event_record: SREventRecord,
    pub uc_index: SRUCI,
    pub srse_index: SRSei,
}

simple_obj!(
    SRGame,
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

impl PPrintable for SRGame {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRGame", |p| {
            p.field("Player").value(&self.player);
            p.field("Ranch").value(&self.ranch);
            p.field("Game settings").value(&self.game_settings);
            p.field("World").value(&self.srw);

            p.field("Item index").value(&self.item_index);
            p.field("Plant index").value(&self.patch_index);
            p.field("Game icon index").value(&self.game_icon_index);
            p.field("Zone index").value(&self.zone_index);
            p.field("Build date").value(&self.build_date);
            p.field("Scene group index").value(&self.scene_group_index);
            p.field("Secret style discs")
                .value(&self.secret_style_discs);
            p.field("Event record").value(&self.event_record);
            p.field("Upgrade component index").value(&self.uc_index);
            p.field("srse index").value(&self.srse_index);

            p.ufield("ped").value(&self.ped);
            p.field("Saved game summary").value(&self.save_game_summary);
            p.ufield("str1").value(&self.str1);
            p.ufield("bytes1").value(&self.bytes1);
            p.ufield("unknown1").value(&self.unknown1);
            p.ufield("unknown2").value(&self.unknown2);
            p.ufield("app").value(&self.app);
            if p.compare_mode {
                let mut srads: Vec<_> = self.srads.clone();
                srads.sort_unstable_by_key(|srad| {
                    srad.as_v2().map(|s| s.index).unwrap_or(-1)
                });
                p.field("Actors").value(&srads);
            } else {
                p.field("Actors").value(&self.srads);
            }
        });
    }
}
