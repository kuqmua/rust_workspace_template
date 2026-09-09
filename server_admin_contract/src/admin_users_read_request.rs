#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminUsersReadRequest {
    search: Option<crate::admin_table_search::AdminTableSearch>,
    pagination: crate::admin_read_page::AdminReadPage,
    select: crate::admin_read_user_selection::AdminReadUserSelection,
    order_by: crate::admin_read_user_order::AdminReadUserOrder,
    where_many: Option<crate::admin_no_body::AdminNoBody>,
}
impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminUsersReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let column = if value.sort().as_ref().is_empty() {
            crate::admin_read_user_column::AdminReadUserColumn::Id(
                crate::admin_no_body::AdminNoBody,
            )
        } else {
            match crate::admin_table_sort_field::AdminTableSortField::try_from_key(
                &crate::admin_table_sort_field::AdminTableSortField::USER,
                crate::admin_table_sort_key_ref::AdminTableSortKeyRef::from(value.sort().as_ref()),
            )? {
                crate::admin_table_sort_field::AdminTableSortField::UserLogin => crate::admin_read_user_column::AdminReadUserColumn::Login(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::UserDisplayName => crate::admin_read_user_column::AdminReadUserColumn::DisplayName(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::UserId => crate::admin_read_user_column::AdminReadUserColumn::Id(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::UserStatus => crate::admin_read_user_column::AdminReadUserColumn::IsBanned(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::AuditAction
                | crate::admin_table_sort_field::AdminTableSortField::AuditCreatedAt
                | crate::admin_table_sort_field::AdminTableSortField::AuditResource
                | crate::admin_table_sort_field::AdminTableSortField::AuditSucceeded
                | crate::admin_table_sort_field::AdminTableSortField::AuditUserId
                | crate::admin_table_sort_field::AdminTableSortField::PermissionId
                | crate::admin_table_sort_field::AdminTableSortField::PermissionName
                | crate::admin_table_sort_field::AdminTableSortField::RoleId
                | crate::admin_table_sort_field::AdminTableSortField::RoleName
                | crate::admin_table_sort_field::AdminTableSortField::RoleSystem => return Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown),
            }
        };
        let order = if value.sort().as_ref().is_empty() {
            crate::admin_sort_direction::AdminSortDirection::Ascending
        } else {
            value.direction()
        };
        Ok(Self::new(
            Some(value.search().clone()),
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_read_user_selection::AdminReadUserSelection::default(),
            crate::admin_read_user_order::AdminReadUserOrder::new(column, order),
            None,
        ))
    }
}
