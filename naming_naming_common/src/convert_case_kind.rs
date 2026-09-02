#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct ConvertCaseKind(convert_case::Case<'static>);
