use crate::{pprint::{PPrintable, Printer}, SRPED, objects::Obj};

impl PPrintable for SRPED {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(self.a)?;
            p.field("Index")?.value(&self.index)
        })
    }
}
