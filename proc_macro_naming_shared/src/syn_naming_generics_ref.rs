#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct SynNamingGenericsRef<'generics_lt>(&'generics_lt syn::Generics);
