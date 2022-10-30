use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRRGI, SRGAME,
};

impl PPrintable<SRGAME> for SRRGI {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("Patches")?.value(&self.patches))
    }
}
