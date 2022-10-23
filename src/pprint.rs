mod printer;
pub use printer::*;
mod primitives;
mod objects;

pub trait PPrintable {
    fn pprint(&self, printer: &mut Printer);
}
impl<P: PPrintable> PPrintable for &P {
    fn pprint(&self, printer: &mut Printer) {
        P::pprint(&self, printer)
    }
}
impl<A: PPrintable, B: PPrintable> PPrintable for (A, B) {
    fn pprint(&self, printer: &mut Printer) {
        printer.print("(");
        self.0.pprint(printer);
        printer.print(", ");
        self.1.pprint(printer);
        printer.print(")");
    }
}
impl<P: PPrintable> PPrintable for Vec<P> {
    fn pprint(&self, printer: &mut Printer) {
        printer.list(self, |p, item| {
            item.pprint(p);
        })
    }
}