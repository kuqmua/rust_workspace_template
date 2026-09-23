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
pub struct AdminPermissionActionsReadRequest {
    #[constructor(order = 3)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 0)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 1)]
    select: crate::admin_read_permission_action_selection::AdminReadPermissionActionSelection,
    #[constructor(order = 2)]
    order_by: crate::admin_read_permission_action_order::AdminReadPermissionActionOrder,
}
impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminPermissionActionsReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let column = match value.sort().as_ref() {
            constants_str::EMPTY | constants_str::SQL_NAMES_ID => crate::admin_read_permission_action_column::AdminReadPermissionActionColumn::Id(crate::admin_no_body::AdminNoBody),
            constants_str::PERMISSION_ACTION_KEY => crate::admin_read_permission_action_column::AdminReadPermissionActionColumn::Key(crate::admin_no_body::AdminNoBody),
            _ => return Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown),
        };
        Ok(Self::new(
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_read_permission_action_selection::AdminReadPermissionActionSelection::default(),
            crate::admin_read_permission_action_order::AdminReadPermissionActionOrder::new(column, value.direction()),
            None,
        ))
    }
}
