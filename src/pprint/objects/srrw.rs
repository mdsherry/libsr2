use crate::{pprint::PPrintable, Srrw, objects::Obj, Printer};

impl PPrintable for Srrw {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("ts")?.value(self.ts)?;
            p.ufield("c")?.value(self.c)
        })
    }
}