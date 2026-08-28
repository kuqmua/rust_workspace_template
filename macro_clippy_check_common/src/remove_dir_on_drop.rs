#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the crate-root Drop adapter owns filesystem cleanup for this private domain guard"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RemoveDirOnDrop {
    path: std::path::PathBuf,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RemoveDirOnDrop {
    #[allow(
        clippy::single_call_fn,
        reason = "constructor keeps private field initialization inside the domain type"
    )]
    pub(crate) const fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}
