#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AttrIdentifierName<'name_lt>(&'name_lt str);
