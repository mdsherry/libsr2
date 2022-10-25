use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRUPGRADECOMPONENTS,
};

impl PPrintable for SRUPGRADECOMPONENTS {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("bytes")?.value(&self.bytes))
    }
}
