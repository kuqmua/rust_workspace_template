#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_not_inner::NotInner,
)]
pub struct IsProjectCommit(bool);
