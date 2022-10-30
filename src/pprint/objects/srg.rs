use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRG, SRGAME,
};

impl PPrintable<SRGAME> for SRG {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Position")?.value(&self.pos)?;
            p.field("Facing")?.value(&self.facing)?;
            p.field("Fed")?.value(self.fed)?;
            p.field("Found")?.value(self.found)?;
            p.field("Gordo type")?.value(&self.gordo_type)?;
            p.ufield("unknown")?.value(&self.unknown)
        })
    }
}
