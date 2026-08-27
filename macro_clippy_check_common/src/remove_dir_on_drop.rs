#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the crate-root Drop adapter owns filesystem cleanup for this private domain guard"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct RemoveDirOnDrop {
    pub(crate) path: std::path::PathBuf,
}
