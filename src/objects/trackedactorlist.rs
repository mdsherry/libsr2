use super::simple_obj;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TrackedActorList {
    pub actor_ids: Vec<f64>,
}
simple_obj!(TrackedActorList, "TRACKEDACTORLIST", actor_ids);
