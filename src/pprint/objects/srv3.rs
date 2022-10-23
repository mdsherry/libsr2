use crate::{pprint::{PPrintable, Printer}, Srv3};

impl PPrintable for Srv3 {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.print(&format!("{self:?}"))
    }
}
