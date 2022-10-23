use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRSE,
};

impl PPrintable for SRSE {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown")?.value(&self.unknown)?;
            p.ufield("ts")?.value(&self.ts)
        })
    }
}
