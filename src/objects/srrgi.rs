use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Srrgi {
    pub patches: Vec<String>,
}
simple_obj!(Srrgi, "SRRGI", patches);
