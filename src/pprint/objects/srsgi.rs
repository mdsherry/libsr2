use crate::{
    pprint::{PPrintable, Printer},
    Obj, SRSGI,
};

impl PPrintable for SRSGI {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Scene groups")?.value(&self.scene_groups)
        })
    }
}
