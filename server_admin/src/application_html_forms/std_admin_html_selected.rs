use super::{
    ADMIN_HTML_FORM_SELECTED_MAX_ITEMS, AdminHtmlFormKey, AdminHtmlFormText,
    StdAdminHtmlSelectedError,
};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(
    from = "bounded_types::domain_types::btree::BoundedBTreeMap<AdminHtmlFormKey, AdminHtmlFormText, ADMIN_HTML_FORM_SELECTED_MAX_ITEMS>"
)]
pub(super) struct StdAdminHtmlSelected(
    bounded_types::domain_types::btree::BoundedBTreeMap<
        AdminHtmlFormKey,
        AdminHtmlFormText,
        ADMIN_HTML_FORM_SELECTED_MAX_ITEMS,
    >,
);
impl TryFrom<std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>>
    for StdAdminHtmlSelected
{
    type Error = StdAdminHtmlSelectedError;
    fn try_from(
        value: std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>,
    ) -> Result<Self, Self::Error> {
        bounded_types::domain_types::btree::BoundedBTreeMap::try_from(value)
            .map(Self)
            .map_err(StdAdminHtmlSelectedError::from)
    }
}
