#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(crate) struct InitEntries(
    bounded_types::bounded_vec::BoundedVec<
        crate::initialization_entry::InitializationEntry,
        0,
        { usize::MAX },
    >,
);
