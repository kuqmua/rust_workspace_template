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
pub struct AdminRolesReadRequest {
    search: Option<crate::admin_table_search::AdminTableSearch>,
    pagination: crate::admin_read_page::AdminReadPage,
    select: crate::admin_read_role_selection::AdminReadRoleSelection,
    order_by: crate::admin_read_role_order::AdminReadRoleOrder,
    where_many: Option<crate::admin_no_body::AdminNoBody>,
}
impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminRolesReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let column = if value.sort().as_ref().is_empty() {
            crate::admin_read_role_column::AdminReadRoleColumn::Id(
                crate::admin_no_body::AdminNoBody,
            )
        } else {
            match crate::admin_table_sort_field::AdminTableSortField::try_from_key(
                &crate::admin_table_sort_field::AdminTableSortField::ROLE,
                crate::admin_table_sort_key_ref::AdminTableSortKeyRef::from(value.sort().as_ref()),
            )? {
                crate::admin_table_sort_field::AdminTableSortField::RoleName => crate::admin_read_role_column::AdminReadRoleColumn::Name(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::RoleId => crate::admin_read_role_column::AdminReadRoleColumn::Id(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::RoleSystem => crate::admin_read_role_column::AdminReadRoleColumn::IsSystem(crate::admin_no_body::AdminNoBody),
                crate::admin_table_sort_field::AdminTableSortField::AuditAction
                | crate::admin_table_sort_field::AdminTableSortField::AuditCreatedAt
                | crate::admin_table_sort_field::AdminTableSortField::AuditResource
                | crate::admin_table_sort_field::AdminTableSortField::AuditSucceeded
                | crate::admin_table_sort_field::AdminTableSortField::AuditUserId
                | crate::admin_table_sort_field::AdminTableSortField::PermissionId
                | crate::admin_table_sort_field::AdminTableSortField::PermissionName
                | crate::admin_table_sort_field::AdminTableSortField::UserId
                | crate::admin_table_sort_field::AdminTableSortField::UserLogin
                | crate::admin_table_sort_field::AdminTableSortField::UserDisplayName
                | crate::admin_table_sort_field::AdminTableSortField::UserStatus => return Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown),
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
            crate::admin_read_role_selection::AdminReadRoleSelection::default(),
            crate::admin_read_role_order::AdminReadRoleOrder::new(column, order),
            None,
        ))
    }
}
