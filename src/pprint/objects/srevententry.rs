use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SREVENTENTRY, SRGAME,
};

impl PPrintable<SRGAME> for SREVENTENTRY {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Type")?.value(&self.typ)?;
            p.field("Info")?.value(&self.info)?;
            p.field("Count")?.value(self.count)?;
            p.field("First updated")?.value(&self.first_updated)?;
            p.field("Last updated")?.value(self.last_updated)?;
            p.field("First updated (wall time)")?
                .value(&self.first_updated_walltime)?;
            p.field("Last updated (wall time)")?
                .value(&self.last_updated_walltime)
        })
    }
}
