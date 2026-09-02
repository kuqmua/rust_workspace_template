#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_foundation::AsRefInner,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::ToTokens,
)]
pub(crate) struct SynExpr(syn::Expr);
