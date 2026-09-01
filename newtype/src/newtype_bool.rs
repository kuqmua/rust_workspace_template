#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype_foundation::FromInner,
    newtype_foundation::GetInner,
)]
#[accessor(pub(crate))]
#[borrow]
pub(crate) struct NewtypeBool(bool);
