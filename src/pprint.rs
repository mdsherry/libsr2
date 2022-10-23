mod printer;
pub use printer::*;
mod objects;
mod primitives;

pub trait PPrintable {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()>;
}
impl<P: PPrintable> PPrintable for &P {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        P::pprint(&self, printer)
    }
}
impl<A: PPrintable, B: PPrintable> PPrintable for (A, B) {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.print("(")?;
        self.0.pprint(printer)?;
        printer.print(", ")?;
        self.1.pprint(printer)?;
        printer.print(")")?;
        Ok(())
    }
}
impl<P: PPrintable> PPrintable for Vec<P> {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.list(self, |p, item| item.pprint(p))
    }
}
