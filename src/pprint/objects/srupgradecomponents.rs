use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRUPGRADECOMPONENTS, SRGAME,
};

impl PPrintable<SRGAME> for SRUPGRADECOMPONENTS {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| p.ufield("bytes")?.value(&self.bytes))
    }
}
