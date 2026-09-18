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
pub struct AdminRolePermissionsReadRequest {
    #[constructor(order = 3)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 1)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 0)]
    select: crate::admin_read_role_permission_selection::AdminReadRolePermissionSelection,
    #[constructor(order = 2)]
    order_by: crate::admin_read_role_permission_order::AdminReadRolePermissionOrder,
}

impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminRolePermissionsReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;

    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        crate::admin_unsearchable_read_query::AdminUnsearchableReadQuery::try_from(value).map(
            |admin_unsearchable_read_query| {
                Self::new(
                    crate::admin_read_role_permission_selection::AdminReadRolePermissionSelection::default(),
                    *admin_unsearchable_read_query.get_pagination(),
                    crate::admin_read_role_permission_order::AdminReadRolePermissionOrder::new(
                        crate::admin_read_role_permission_column::AdminReadRolePermissionColumn::CreatedAt(
                            crate::admin_no_body::AdminNoBody,
                        ),
                        admin_unsearchable_read_query.get_order(),
                    ),
                    None,
                )
            },
        )
    }
}
