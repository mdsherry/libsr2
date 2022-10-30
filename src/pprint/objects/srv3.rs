use crate::{
    pprint::{PPrintable, Printer},
    SRV3
};

impl<T> PPrintable<T> for SRV3 {
    fn pprint(&self, printer: &mut Printer<T>) -> std::io::Result<()> {
        printer.print(&format!("{self:?}"))
    }
}
