use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRAPP, SRGAME,
};

impl PPrintable<SRGAME> for SRAPP {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("item_map")?.value(&self.item_map)?;
            p.ufield("unknown")?.value(&self.unknown)
        })
    }
}
