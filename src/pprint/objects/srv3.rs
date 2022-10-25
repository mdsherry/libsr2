use crate::{
    pprint::{PPrintable, Printer},
    SRV3,
};

impl PPrintable for SRV3 {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.print(&format!("{self:?}"))
    }
}
