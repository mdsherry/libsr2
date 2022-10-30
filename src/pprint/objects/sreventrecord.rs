use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SREVENTRECORD, SRGAME,
};

impl PPrintable<SRGAME> for SREVENTRECORD {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("events")?.value(&self.events))
    }
}
