#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the tool-command owner mutates this private process wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct ProcessCommand(std::process::Command);
