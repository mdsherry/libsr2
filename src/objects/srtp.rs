use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRTp {
    pub a: i32,
    pub b: i32,
}
simple_obj!(SRTp, "SRTP", a, b);
