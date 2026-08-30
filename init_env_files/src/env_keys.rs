#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(crate) struct EnvKeys(
    bounded_types::bounded_vec::BoundedVec<crate::env_key::EnvKey, 0, { usize::MAX }>,
);
