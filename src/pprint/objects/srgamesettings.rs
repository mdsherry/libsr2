use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGameSettings,
};

impl PPrintable for SRGameSettings {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Game settings")?.value(&self.game_settings)?;
            p.field("Game icon")?.value(self.game_icon)
        })
    }
}
