#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(
    from = "bounded_types::bounded_b_tree_map::BoundedBTreeMap<crate::admin_html_form_key::AdminHtmlFormKey, crate::admin_html_form_text::AdminHtmlFormText, { crate::admin_html_form_selected_max_items::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS }>"
)]
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
        value: std::collections::BTreeMap<
            crate::admin_html_form_key::AdminHtmlFormKey,
            crate::admin_html_form_text::AdminHtmlFormText,
        >,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_b_tree_map::BoundedBTreeMap::try_from(value)
            .map(Self)
            .map_err(crate::std_admin_html_selected_error::StdAdminHtmlSelectedError::from)
    }
}
