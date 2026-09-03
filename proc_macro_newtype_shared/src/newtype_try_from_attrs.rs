#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct NewtypeTryFromAttrs {
    error: Option<crate::syn_type::SynType>,
    validator: crate::syn_expr::SynExpr,
}
