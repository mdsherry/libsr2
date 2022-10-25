#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSECRETSTYLEDISC {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
}
simple_obj!(SRSECRETSTYLEDISC, "SRSECRETSTYLEDISC", a, b, c, d);
