use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRDZR,
};

impl PPrintable for SRDZR {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("unknown")?.value(&self.unknown))
    }
}
