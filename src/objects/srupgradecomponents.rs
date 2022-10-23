use super::simple_obj;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRUpgradeComponents {
    pub bytes: [u8; 4],
}
simple_obj!(SRUpgradeComponents, "SRUPGRADECOMPONENTS", bytes);
