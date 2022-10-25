use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRRESNODE,
};

impl PPrintable for SRRESNODE {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Name")?.value(&self.name)?;
            p.field("Resource type")?.value(&self.resource_type)?;
            p.field("Contents")?.value(&self.contents)?;
            p.ufield("Count 1")?.value(self.count1)?;
            p.ufield("Count 2")?.value(self.count2)?;
            p.ufield("Bytes")?.value(&self.ts)
        })
    }
}
