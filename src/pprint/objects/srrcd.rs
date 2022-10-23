use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRRCD,
};

impl PPrintable for SRRCD {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Condition")?.value(self.condition)?;
            p.field("State transition time")?.value(self.time)
        })
    }
}
