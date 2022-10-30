use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRPU, SRGAME,
};

impl PPrintable<SRGAME> for SRPU {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Purchased upgrades")?
                .map(&self.map.0, |p, (k, v)| p.print(&format!("{}: {}", k, v)))
        })
    }
}
