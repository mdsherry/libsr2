use crate::{objects::Obj, pprint::PPrintable, Printer, SRRW};

impl PPrintable for SRRW {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("ts")?.value(self.ts)?;
            p.ufield("c")?.value(self.c)
        })
    }
}
