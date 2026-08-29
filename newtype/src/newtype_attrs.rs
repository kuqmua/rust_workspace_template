#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module parses and consumes this domain model
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct NewtypeAttrs {
    options: workspace_macro_helpers::domain_types::UniqueOptionBTreeSet<crate::NewtypeOption>,
    try_from: Option<crate::NewtypeTryFromAttrs>,
    to_err_string_mode: Option<crate::ToErrStringMode>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NewtypeAttrs {
    pub(crate) fn contains(&self, option: crate::NewtypeOption) -> crate::NewtypeBool {
        crate::NewtypeBool::from(self.options.contains(option).get())
    }
}
