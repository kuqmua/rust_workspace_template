#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
#[accessor(pub(crate))]
#[borrow]
pub(crate) struct NewtypeBool(bool);
