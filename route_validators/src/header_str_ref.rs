#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(crate) struct HeaderStrRef<'header_str_lt>(&'header_str_lt str);
