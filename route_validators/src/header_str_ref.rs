#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(crate) struct HeaderStrRef<'header_str_lt>(&'header_str_lt str);
