use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRAPP,
};

impl PPrintable for SRAPP {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("item_map")?.value(&self.item_map)?;
            p.ufield("unknown")?.value(&self.unknown)
        })
    }
}
