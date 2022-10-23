use crate::{pprint::{PPrintable, Printer}, StringPair};

impl PPrintable for StringPair {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        (&self.0, &self.1).pprint(printer)
    }
}
