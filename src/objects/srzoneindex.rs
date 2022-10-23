use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRZoneIndex {
    pub zones: Vec<String>,
}
simple_obj!(SRZoneIndex, "SRGZONEINDEX", zones);
