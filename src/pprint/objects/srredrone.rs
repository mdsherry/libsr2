use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRREDRONE, SRGAME,
};

impl PPrintable<SRGAME> for SRREDRONE {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Discovered")?.value(self.discovered)?;
            p.field("Name")?.value(&self.name)
        })
    }
}
