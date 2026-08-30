use bounded_types::bounded_vec::BoundedVec;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(crate) struct InitEntries(
    BoundedVec<crate::initialization_entry::InitializationEntry, 0, { usize::MAX }>,
);
