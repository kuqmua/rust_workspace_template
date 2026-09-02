#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(crate) struct StdBool(bool);
