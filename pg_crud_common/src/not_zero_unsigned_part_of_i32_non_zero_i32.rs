#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
pub(super) struct NotZeroUnsignedPartOfI32NonZeroI32(pub(super) std::num::NonZeroI32);
