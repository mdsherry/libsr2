use crate::{pprint::{PPrintable, Printer}, SRGame, objects::Obj};

impl PPrintable for SRGame {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Player")?.value(&self.player)?;
            p.field("Ranch")?.value(&self.ranch)?;
            p.field("Game settings")?.value(&self.game_settings)?;
            p.field("World")?.value(&self.srw)?;

            p.field("Item index")?.value(&self.item_index)?;
            p.field("Plant index")?.value(&self.patch_index)?;
            p.field("Game icon index")?.value(&self.game_icon_index)?;
            p.field("Zone index")?.value(&self.zone_index)?;
            p.field("Build date")?.value(&self.build_date)?;
            p.field("Scene group index")?.value(&self.scene_group_index)?;
            p.field("Secret style discs")?
                .value(&self.secret_style_discs)?;
            p.field("Event record")?.value(&self.event_record)?;
            p.field("Upgrade component index")?.value(&self.uc_index)?;
            p.field("srse index")?.value(&self.srse_index)?;

            p.ufield("ped")?.value(&self.ped)?;
            p.field("Saved game summary")?.value(&self.save_game_summary)?;
            p.ufield("str1")?.value(&self.str1)?;
            p.ufield("bytes1")?.value(&self.bytes1)?;
            p.ufield("unknown1")?.value(&self.unknown1)?;
            p.ufield("unknown2")?.value(&self.unknown2)?;
            p.ufield("app")?.value(&self.app)?;
            if p.compare_mode() {
                let mut srads: Vec<_> = self.srads.clone();
                srads.sort_unstable_by_key(|srad| {
                    srad.as_v2().map(|s| s.index).unwrap_or(-1)
                });
                p.field("Actors")?.value(&srads)
            } else {
                p.field("Actors")?.value(&self.srads)
            }
        })
    }
}
