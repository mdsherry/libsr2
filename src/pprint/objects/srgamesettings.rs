use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGAMESETTINGS, SRGAME,
};

impl PPrintable<SRGAME> for SRGAMESETTINGS {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Game settings")?.value(&self.game_settings)?;
            p.field("Game icon")?.value(self.game_icon)
        })
    }
}
