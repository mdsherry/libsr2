mod printer;
pub use printer::*;
mod objects;
mod primitives;

pub trait PPrintable<T> {
    fn pprint(&self, printer: &mut Printer<T>) -> std::io::Result<()>;
}
impl<T, P: PPrintable<T>> PPrintable<T> for &P {
    fn pprint(&self, printer: &mut Printer<T>) -> std::io::Result<()> {
        P::pprint(self, printer)
    }
}
impl<T, A: PPrintable<T>, B: PPrintable<T>> PPrintable<T> for (A, B) {
    fn pprint(&self, printer: &mut Printer<T>) -> std::io::Result<()> {
        printer.print("(")?;
        self.0.pprint(printer)?;
        printer.print(", ")?;
        self.1.pprint(printer)?;
        printer.print(")")?;
        Ok(())
    }
}
impl<T, P: PPrintable<T>> PPrintable<T> for Vec<P> {
    fn pprint(&self, printer: &mut Printer<T>) -> std::io::Result<()> {
        printer.list(self, |p, item| item.pprint(p))
    }
}


impl<T, P: PPrintable<T>> PPrintable<T> for Option<P> {
    fn pprint(&self, printer: &mut Printer<T>) -> std::io::Result<()> {
        if let Some(value) = self {
            value.pprint(printer)?;
        } else {
            printer.print("<none>")?;
        }
        Ok(())
    }
}
