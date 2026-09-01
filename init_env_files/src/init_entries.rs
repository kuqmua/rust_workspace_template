#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub(crate) struct InitEntries(
    bounded_types::bounded_vec::BoundedVec<
        crate::initialization_entry::InitializationEntry,
        0,
        { usize::MAX },
    >,
);
