#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
)]
#[getters(bare)]
#[derive(proc_macro_new::New)]
pub(crate) struct AdminCsrQuery {
    direction: Option<server_admin_contract::admin_text::AdminText>,
    filter_end: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    filter_field: Option<server_admin_contract::admin_filter_field::AdminFilterField>,
    filter_operation:
        Option<server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey>,
    filter_value: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    #[getters(copy)]
    limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    #[getters(copy)]
    offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    search: server_admin_contract::admin_table_search::AdminTableSearch,
    sort: server_admin_contract::admin_table_sort_key::AdminTableSortKey,
    #[getters(copy)]
    table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
}
impl AdminCsrQuery {
    pub(crate) fn from_location() -> Result<Self, crate::admin_table_load_error::AdminTableLoadError>
    {
        let window =
            web_sys::window().ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let search = window
            .location()
            .search()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let params = web_sys::UrlSearchParams::new_with_str(&search)
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let pathname = window
            .location()
            .pathname()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let table = server_admin_contract::admin_data_table::AdminDataTable::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        Ok(Self::new(
            params
                .get(constants_str::ADMIN_DIRECTION_QUERY_KEY)
                .map(server_admin_contract::admin_text::AdminText::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_END_QUERY_KEY)
                .map(server_admin_contract::admin_filter_value::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_FIELD_QUERY_KEY)
                .map(server_admin_contract::admin_filter_field::AdminFilterField::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_OPERATION_QUERY_KEY)
                .map(server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_VALUE_QUERY_KEY)
                .map(server_admin_contract::admin_filter_value::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_LIMIT_QUERY_KEY)
                .and_then(|value| value.parse::<u16>().ok())
                .and_then(|value| {
                    server_admin_contract::admin_page_limit::AdminPageLimit::try_from(value).ok()
                })
                .unwrap_or_default(),
            params
                .get(constants_str::ADMIN_OFFSET_QUERY_KEY)
                .and_then(|value| value.parse::<u32>().ok())
                .map_or_else(
                    server_admin_contract::admin_page_offset::AdminPageOffset::default,
                    server_admin_contract::admin_page_offset::AdminPageOffset::from,
                ),
            params
                .get(constants_str::ADMIN_SEARCH_QUERY_KEY)
                .map(server_admin_contract::admin_table_search::AdminTableSearch::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            params
                .get(constants_str::ADMIN_SORT_QUERY_KEY)
                .map(server_admin_contract::admin_table_sort_key::AdminTableSortKey::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            table,
        ))
    }
}
