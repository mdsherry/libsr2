use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRITI, SRGAME,
};

impl PPrintable<SRGAME> for SRITI {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("identifiable_types")?
                .value(&self.identifiable_types)
        })
    }
}
