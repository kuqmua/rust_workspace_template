#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default, proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub(crate) struct BoundedStringAttrs {
    description: Option<crate::syn_expr::SynExpr>,
    max: Option<crate::syn_expr::SynExpr>,
    min: Option<crate::syn_expr::SynExpr>,
    options: workspace_macro_helpers::unique_option_b_tree_set::UniqueOptionBTreeSet<
        crate::bounded_string_option::BoundedStringOption,
    >,
    validator: Option<crate::syn_expr::SynExpr>,
}
