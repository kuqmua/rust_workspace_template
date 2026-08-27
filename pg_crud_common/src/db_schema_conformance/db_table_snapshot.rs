#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbTableSnapshot {
    pub(super) columns: super::DbColumnSnapshots,
    pub(super) objects: super::DbObjectSnapshots,
}

impl DbTableSnapshot {
    #[must_use]
    pub fn new(
        mut columns: super::DbColumnSnapshots,
        mut objects: super::DbObjectSnapshots,
    ) -> Self {
        columns.sort_unstable();
        objects.sort_unstable();
        Self { columns, objects }
    }
}
