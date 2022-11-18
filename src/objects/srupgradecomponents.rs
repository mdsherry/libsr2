#[cfg(test)]
mod test;

use crate::primitives::{VecMap, UpgradeComponentId};

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SRUPGRADECOMPONENTS {
    pub components: VecMap<UpgradeComponentId, i32>,
}
simple_obj!(SRUPGRADECOMPONENTS, "SRUPGRADECOMPONENTS", components);
