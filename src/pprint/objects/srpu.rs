use crate::{pprint::{PPrintable, Printer}, SRPU, objects::Obj};

impl PPrintable for SRPU {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Purchased upgrades")?.map(&self.map.0, |p, (k, v)| {
                p.print(&format!("{}: {}", k, v))
            })
        })
    }
}

