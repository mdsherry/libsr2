use crate::{pprint::{PPrintable, Printer}, SRSed, objects::Obj};

impl PPrintable for SRSed {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown")?.value(&self.unknown)
        })
    }
}
