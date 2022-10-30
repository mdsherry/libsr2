use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRSE, SRGAME,
};

impl PPrintable<SRGAME> for SRSE {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown")?.value(&self.unknown)?;
            p.ufield("ts")?.value(&self.ts)
        })
    }
}
