use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRSEI,
};

impl PPrintable for SRSEI {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("hashes")?.value(&self.hashes))
    }
}
