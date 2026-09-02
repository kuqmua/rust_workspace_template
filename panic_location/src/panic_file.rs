#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(crate) struct PanicFile<'file_lt>(&'file_lt str);
