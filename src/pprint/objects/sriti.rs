use crate::{pprint::{PPrintable, Printer}, SRITI, objects::Obj};

impl PPrintable for SRITI {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("identifiable_types")
                .value(&self.identifiable_types);
        });
    }
}