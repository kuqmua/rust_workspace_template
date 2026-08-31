#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct NewtypeAttrs {
    options: workspace_macro_helpers::unique_option_b_tree_set::UniqueOptionBTreeSet<
        crate::newtype_option::NewtypeOption,
    >,
    try_from: Option<crate::newtype_try_from_attrs::NewtypeTryFromAttrs>,
    to_err_string_mode: Option<crate::to_err_string_mode::ToErrStringMode>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NewtypeAttrs {
    pub(crate) fn contains(
        &self,
        option: crate::newtype_option::NewtypeOption,
    ) -> crate::newtype_bool::NewtypeBool {
        crate::newtype_bool::NewtypeBool::from(self.options.contains(option).get())
    }
}
