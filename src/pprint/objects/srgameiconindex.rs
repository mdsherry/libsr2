use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGAMEICONINDEX, SRGAME,
};

impl PPrintable<SRGAME> for SRGAMEICONINDEX {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Game icons")?.value(&self.game_icons)
        })
    }
}
