use crate::{
    pprint::{PPrintable, Printer},
    StringPair, SRGAME,
};

impl PPrintable<SRGAME> for StringPair {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        (&self.0, &self.1).pprint(printer)
    }
}
