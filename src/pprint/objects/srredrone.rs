use crate::{pprint::{PPrintable, Printer}, SRREDRONE, objects::Obj};

impl PPrintable for SRREDRONE {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Discovered").value(self.discovered);
            p.field("Name").value(&self.name);
        })
    }
}
