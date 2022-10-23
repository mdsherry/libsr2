use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRTp,
};

impl PPrintable for SRTp {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(self.a)?;
            p.ufield("b")?.value(self.b)
        })
    }
}
