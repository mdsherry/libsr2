use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGAMEICONINDEX,
};

impl PPrintable for SRGAMEICONINDEX {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Game icons")?.value(&self.game_icons)
        })
    }
}
