use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRZoneIndex,
};

impl PPrintable for SRZoneIndex {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("zones")?.value(&self.zones))
    }
}
