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
pub struct AdminPermissionResourcesReadRequest {
    #[constructor(order = 3)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 0)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 1)]
    select: crate::admin_read_permission_resource_selection::AdminReadPermissionResourceSelection,
    #[constructor(order = 2)]
    order_by: crate::admin_read_permission_resource_order::AdminReadPermissionResourceOrder,
}
impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminPermissionResourcesReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let column = if matches!(
            value.sort().as_ref(),
            constants_str::EMPTY | constants_str::SQL_NAMES_ID
        ) {
            crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn::Id(
                crate::admin_no_body::AdminNoBody,
            )
        } else if value.sort().as_ref() == constants_str::PERMISSION_ACTION_KEY {
            crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn::Key(
                crate::admin_no_body::AdminNoBody,
            )
        } else {
            return Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown);
        };
        Ok(Self::new(
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_read_permission_resource_selection::AdminReadPermissionResourceSelection::default(),
            crate::admin_read_permission_resource_order::AdminReadPermissionResourceOrder::new(column, value.direction()),
            None,
        ))
    }
}
