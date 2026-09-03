#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(
    from = "bounded_types::bounded_b_tree_map::BoundedBTreeMap<crate::admin_html_form_key::AdminHtmlFormKey, crate::admin_html_form_text::AdminHtmlFormText, { crate::admin_html_form_selected_max_items::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS }>"
)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct StdAdminHtmlSelected(
    bounded_types::bounded_b_tree_map::BoundedBTreeMap<
        crate::admin_html_form_key::AdminHtmlFormKey,
        crate::admin_html_form_text::AdminHtmlFormText,
        { crate::admin_html_form_selected_max_items::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS },
    >,
);
impl
    TryFrom<
        std::collections::BTreeMap<
            crate::admin_html_form_key::AdminHtmlFormKey,
            crate::admin_html_form_text::AdminHtmlFormText,
        >,
    > for StdAdminHtmlSelected
{
    type Error = crate::std_admin_html_selected_error::StdAdminHtmlSelectedError;
    fn try_from(
        b_tree_map: std::collections::BTreeMap<
            crate::admin_html_form_key::AdminHtmlFormKey,
            crate::admin_html_form_text::AdminHtmlFormText,
        >,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_b_tree_map::BoundedBTreeMap::try_from(b_tree_map)
            .map(Self)
            .map_err(crate::std_admin_html_selected_error::StdAdminHtmlSelectedError::from)
    }
}
