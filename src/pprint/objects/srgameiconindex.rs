use crate::{pprint::{PPrintable, Printer}, SRGameIconIndex, objects::Obj};

impl PPrintable for SRGameIconIndex {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Game icons")?.value(&self.game_icons)
        })
    }
}
