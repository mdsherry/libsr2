use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRPED, SRGAME,
};

impl PPrintable<SRGAME> for SRPED {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(self.a)?;
            p.field("Index")?.value(&self.index)
        })
    }
}
