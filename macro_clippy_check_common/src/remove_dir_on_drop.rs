#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the crate-root Drop adapter owns filesystem cleanup for this private domain guard"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct RemoveDirOnDrop {
    path: std::path::PathBuf,
}
