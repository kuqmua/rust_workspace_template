#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    newtype::DerefInner,
    newtype::FromInner,
)]
#[serde(transparent)]
#[derive(generate_accessor::Getters)]
pub(crate) struct AdminPasswordChangeRequired(bool);
