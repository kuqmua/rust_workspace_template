#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_iterator::IntoIterator,
)]
pub(crate) struct EnvKeys(
    bounded_types::bounded_vec::BoundedVec<crate::env_key::EnvKey, 0, { usize::MAX }>,
);
