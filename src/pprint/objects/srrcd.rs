use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRRCD, SRGAME,
};

impl PPrintable<SRGAME> for SRRCD {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Condition")?.value(self.condition)?;
            p.field("State transition time")?.value(self.time)
        })
    }
}
