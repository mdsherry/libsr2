use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRGZONEINDEX, SRGAME,
};

impl PPrintable<SRGAME> for SRGZONEINDEX {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("zones")?.value(&self.zones))
    }
}
