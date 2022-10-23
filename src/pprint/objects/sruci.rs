use crate::{pprint::{PPrintable, Printer}, SRUCI, objects::Obj};

impl PPrintable for SRUCI {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Upgrade components")
                .value(&self.upgrade_components);
        })
    }
}
