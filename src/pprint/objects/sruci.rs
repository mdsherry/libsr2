use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRUCI,
};

impl PPrintable for SRUCI {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Upgrade components")?
                .value(&self.upgrade_components)
        })
    }
}
