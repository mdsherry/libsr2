use super::simple_obj;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSed {
    pub unknown: Vec<(i32, f32)>,
}
simple_obj!(SRSed, "SRSED", unknown);