use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRRGI,
};

impl PPrintable for SRRGI {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("Patches")?.value(&self.patches))
    }
}
