#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(crate) struct EnvKeys(
    bounded_types::bounded_vec::BoundedVec<crate::env_key::EnvKey, 0, { usize::MAX }>,
);
