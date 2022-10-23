use crate::{pprint::{PPrintable, Printer}, SRSGI, Obj};

impl PPrintable for SRSGI {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Scene groups").value(&self.scene_groups);
        })
    }
}
