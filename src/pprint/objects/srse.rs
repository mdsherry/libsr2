use crate::{pprint::{PPrintable, Printer}, SRSE, objects::Obj};

impl PPrintable for SRSE {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown").value(&self.unknown);
            p.ufield("ts").value(&self.ts);
        })
    }
}
