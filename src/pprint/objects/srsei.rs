use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRSEI, SRGAME,
};

impl PPrintable<SRGAME> for SRSEI {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("hashes")?.value(&self.hashes))
    }
}
