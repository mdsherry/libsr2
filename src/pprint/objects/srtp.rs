use crate::{pprint::{PPrintable, Printer}, SRTp, objects::Obj};

impl PPrintable for SRTp {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(self.a)?;
            p.ufield("b")?.value(self.b)
        })
    }
}
