use crate::{pprint::{PPrintable, Printer}, SRAPP, objects::Obj};

impl PPrintable for SRAPP {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("item_map").value(&self.item_map);
            p.ufield("unknown").value(&self.unknown);
        })
    }
}
