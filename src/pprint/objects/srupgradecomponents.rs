use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRUpgradeComponents,
};

impl PPrintable for SRUpgradeComponents {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("bytes")?.value(&self.bytes))
    }
}
