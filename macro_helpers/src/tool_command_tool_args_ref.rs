#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the tool-command owner reads this borrowed arguments wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ToolArgsRef<'lt>(&'lt [&'lt str]);
