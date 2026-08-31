#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct OpenApiSpecificationPath(&'static str);
