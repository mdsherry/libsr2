#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct InGameTime(pub f64);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TimeSinceYear1(pub i64);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VecMap<K, V>(pub Vec<(K, V)>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowsTime(pub i64);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneGroupId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlantId(pub usize);
impl Default for PlantId {
    fn default() -> Self {
        Self(usize::MAX)
    }
}
