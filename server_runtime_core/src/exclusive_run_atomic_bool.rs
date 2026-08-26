#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "exclusive-run owner and guard modules share this private atomic state wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DerefInner, newtype::FromInner,
)]
pub(super) struct ExclusiveRunAtomicBool(std::sync::atomic::AtomicBool);
