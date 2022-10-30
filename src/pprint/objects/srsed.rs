use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRSED, SRGAME,
};

impl PPrintable<SRGAME> for SRSED {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("unknown")?.value(&self.unknown))
    }
}
