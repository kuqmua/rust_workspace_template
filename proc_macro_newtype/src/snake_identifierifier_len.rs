#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
#[accessor(pub(super))]
#[borrow]
pub(crate) struct SnakeIdentifierifierLen(usize);
