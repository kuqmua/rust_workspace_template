#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitializationStatus {
    Created,
    SkippedExisting,
    Updated,
    WouldCreate,
    WouldUpdate,
}
