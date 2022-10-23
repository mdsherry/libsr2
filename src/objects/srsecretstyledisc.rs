use super::simple_obj;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSecretStyleDisc {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
}
simple_obj!(SRSecretStyleDisc, "SRSECRETSTYLEDISC", a, b, c, d);
