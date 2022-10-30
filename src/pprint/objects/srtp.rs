use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRTP, SRGAME,
};

impl PPrintable<SRGAME> for SRTP {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(self.a)?;
            p.ufield("b")?.value(self.b)
        })
    }
}
