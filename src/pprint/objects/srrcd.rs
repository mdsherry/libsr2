use crate::{pprint::{PPrintable, Printer}, SRRCD, objects::Obj};

impl PPrintable for SRRCD {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Condition")?.value(self.condition)?;
            p.field("State transition time")?.value(self.time)
        })
    }
}
