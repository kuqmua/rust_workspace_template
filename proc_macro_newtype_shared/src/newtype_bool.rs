#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
    proc_macro_newtype_foundation_foundation_get_inner::GetInner,
)]
#[accessor(pub(crate))]
#[borrow]
pub(crate) struct NewtypeBool(bool);
