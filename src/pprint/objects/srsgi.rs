use crate::{
    pprint::{PPrintable, Printer},
    Obj, SRSGI, SRGAME,
};

impl PPrintable<SRGAME> for SRSGI {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Scene groups")?.value(&self.scene_groups)
        })
    }
}
