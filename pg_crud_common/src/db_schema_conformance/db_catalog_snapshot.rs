#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DbCatalogSnapshot {
    pub(super) objects: super::DbObjectSnapshots,
}

impl DbCatalogSnapshot {
    #[must_use]
    pub fn new(mut objects: super::DbObjectSnapshots) -> Self {
        objects.sort_unstable();
        Self { objects }
    }
}
