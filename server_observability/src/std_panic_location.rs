#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the observed-error owner module reads the private panic-location wrapper for deterministic tests"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct StdPanicLocation(&'static std::panic::Location<'static>);
