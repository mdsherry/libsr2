use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGZONEINDEX,
};

impl PPrintable for SRGZONEINDEX {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("zones")?.value(&self.zones))
    }
}
