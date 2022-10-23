use crate::{pprint::{PPrintable, Printer}, SRUpgradeComponents, objects::Obj};

impl PPrintable for SRUpgradeComponents {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.ufield("bytes").value(&self.bytes);
        });
    }
}