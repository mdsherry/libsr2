use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SREventRecord,
};

impl PPrintable for SREventRecord {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.field("events")?.value(&self.events))
    }
}
