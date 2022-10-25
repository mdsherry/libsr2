use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRSECRETSTYLEDISC,
};

impl PPrintable for SRSECRETSTYLEDISC {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(self.a)?;
            p.ufield("b")?.value(self.b)?;
            p.ufield("c")?.value(self.c)?;
            p.ufield("d")?.value(self.d)
        })
    }
}
