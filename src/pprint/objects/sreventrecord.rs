use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SREVENTRECORD,
};

impl PPrintable for SREVENTRECORD {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("events")?.value(&self.events))
    }
}
