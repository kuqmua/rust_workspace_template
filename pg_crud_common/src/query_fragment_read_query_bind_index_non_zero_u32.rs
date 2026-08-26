#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the query-fragment owner renders this private validated bind index"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ReadQueryBindIndexNonZeroU32(std::num::NonZeroU32);
