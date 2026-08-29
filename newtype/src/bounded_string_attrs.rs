#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module incrementally builds this parsed domain model
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct BoundedStringAttrs {
    description: Option<crate::SynExpr>,
    max: Option<crate::SynExpr>,
    min: Option<crate::SynExpr>,
    options:
        workspace_macro_helpers::domain_types::UniqueOptionBTreeSet<crate::BoundedStringOption>,
    validator: Option<crate::SynExpr>,
}
