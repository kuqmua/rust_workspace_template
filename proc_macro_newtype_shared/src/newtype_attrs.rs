#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "newtype attrs keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(proc_macro_getters::Getters)]
#[getters(get_mut)]
pub(crate) struct NewtypeAttrs {
    options: workspace_macro_helpers::unique_option_b_tree_set::UniqueOptionBTreeSet<
        crate::newtype_option::NewtypeOption,
    >,
    to_err_string_mode: Option<crate::to_err_string_mode::ToErrStringMode>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NewtypeAttrs {
    pub(crate) fn contains(
        &self,
        newtype_option: crate::newtype_option::NewtypeOption,
    ) -> crate::newtype_bool::NewtypeBool {
        crate::newtype_bool::NewtypeBool::from(self.options.contains(newtype_option).get())
    }
}
