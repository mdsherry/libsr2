use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGSUMM, SRGAME,
};

impl PPrintable<SRGAME> for SRGSUMM {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Build")?.value(&self.build)?;
            p.field("Time")?.value(&self.time)?;
            p.field("Save time")?.value(&self.save_datetime)?;
            p.field("Money")?.value(&self.money)?;
            p.field("Discoveries")?.value(&self.discoveries)?;
            p.field("Game icon")?.value(&self.game_icon)?;
            p.ufield("unknown")?.value(&self.unknown)
        })
    }
}
