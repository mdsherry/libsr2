#[cfg(test)]
mod test;

use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TRACKEDACTORLIST {
    pub actor_ids: Vec<f64>,
}
simple_obj!(TRACKEDACTORLIST, "TRACKEDACTORLIST", actor_ids);
