use crate::{pprint::{PPrintable, Printer}, Srrgi, objects::Obj};

impl PPrintable for Srrgi {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Patches").value(&self.patches);
        });
    }
}