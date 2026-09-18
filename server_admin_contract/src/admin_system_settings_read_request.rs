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
pub struct AdminSystemSettingsReadRequest {
    #[constructor(order = 4)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 0)]
    search: Option<crate::admin_table_search::AdminTableSearch>,
    #[constructor(order = 2)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 1)]
    select: crate::admin_read_system_settings_selection::AdminReadSystemSettingsSelection,
    #[constructor(order = 3)]
    order_by: crate::admin_read_system_settings_order::AdminReadSystemSettingsOrder,
}

impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminSystemSettingsReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;

    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        if !value.sort().as_ref().is_empty() {
            return Err(
                crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown,
            );
        }
        Ok(Self::new(
            Some(value.search().clone()),
            crate::admin_read_system_settings_selection::AdminReadSystemSettingsSelection::default(
            ),
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_read_system_settings_order::AdminReadSystemSettingsOrder::new(
                crate::admin_read_system_settings_column::AdminReadSystemSettingsColumn::Id(
                    crate::admin_no_body::AdminNoBody,
                ),
                crate::admin_sort_direction::AdminSortDirection::Ascending,
            ),
            None,
        ))
    }
}
