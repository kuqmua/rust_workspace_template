#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub struct BytesBodyBytes(pub(super) bytes::Bytes);
