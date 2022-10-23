use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRREDRONE {
    pub discovered: bool,
    pub name: String,
}
simple_obj!(SRREDRONE, "SRREDRONE", discovered, name);
