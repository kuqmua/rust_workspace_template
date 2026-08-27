#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::FromInner,
    newtype::ToErrString,
)]
#[serde(from = "usize")]
pub struct BodySizeLimitBytes(pub(super) usize);
