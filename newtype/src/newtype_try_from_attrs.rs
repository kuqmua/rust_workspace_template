#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub(crate) struct NewtypeTryFromAttrs {
    error: Option<crate::syn_type::SynType>,
    validator: crate::syn_expr::SynExpr,
}
