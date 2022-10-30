use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRUCI, SRGAME,
};

impl PPrintable<SRGAME> for SRUCI {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Upgrade components")?
                .value(&self.upgrade_components)
        })
    }
}
