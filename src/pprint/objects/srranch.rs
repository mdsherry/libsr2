use crate::{
    pprint::{PPrintable, Printer},
    SRRANCH,
};

impl PPrintable for SRRANCH {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object("SRRANCH", |p| {
            p.field("Plots")?.value(&self.plots)?;
            p.ufield("Doors")?.value(&self.doors)?;
            p.ufield("map2")?.value(&self.map2)?;
            p.ufield("map3")?.value(&self.map3)
        })
    }
}
