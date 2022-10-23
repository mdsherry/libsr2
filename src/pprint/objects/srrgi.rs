use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    Srrgi,
};

impl PPrintable for Srrgi {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("Patches")?.value(&self.patches))
    }
}
