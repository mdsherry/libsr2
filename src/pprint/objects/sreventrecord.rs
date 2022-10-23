use crate::{pprint::{PPrintable, Printer}, SREventRecord, objects::Obj};

impl PPrintable for SREventRecord {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("events").value(&self.events);
        })
    }
}
