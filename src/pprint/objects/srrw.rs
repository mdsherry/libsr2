use crate::{pprint::PPrintable, Srrw, objects::Obj};

impl PPrintable for Srrw {
    fn pprint(&self, printer: &mut crate::pprint::Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("ts").value(self.ts);
            p.ufield("c").value(self.c);
        });
    }
}