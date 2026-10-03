#[must_use]
pub fn admin_path_uses_table_query(
    admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
) -> crate::admin_bool::AdminBool {
    crate::admin_bool::AdminBool::from(
        crate::admin_data_table::AdminDataTable::from_frontend_path(admin_page_path_ref).is_some()
            || crate::admin_page::AdminPage::from_path(admin_page_path_ref)
                .is_some_and(|admin_page| bool::from(admin_page.uses_table_query())),
    )
}
