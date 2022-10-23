use crate::{pprint::{PPrintable, Printer}, SRZoneIndex, objects::Obj};

impl PPrintable for SRZoneIndex {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("zones").value(&self.zones);
        })
    }
}