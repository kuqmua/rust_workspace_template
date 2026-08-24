impl super::AdminCsrQuery {
    pub(in crate::app) fn api_url(
        &self,
    ) -> Result<Option<crate::app::http::url::AdminCsrApiUrl>, crate::app::state::AdminTableLoadError>
    {
        let Some(table) = self.table else {
            return Ok(None);
        };
        let search = web_sys::window()
            .ok_or(crate::app::state::AdminTableLoadError::Fetch)?
            .location()
            .search()
            .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
        crate::app::http::url::admin_api_url_with_suffix(
            server_admin_contract::AdminRoute::DataTable(table),
            crate::app::http::url::AdminCsrApiUrlSuffixRef::from(search.as_str()),
        )
        .map(Some)
    }

    pub(in crate::app) fn from_location() -> Result<Self, crate::app::state::AdminTableLoadError> {
        let window = web_sys::window().ok_or(crate::app::state::AdminTableLoadError::Fetch)?;
        let search = window
            .location()
            .search()
            .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
        let params = web_sys::UrlSearchParams::new_with_str(&search)
            .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
        let pathname = window
            .location()
            .pathname()
            .map_err(|_error| crate::app::state::AdminTableLoadError::Fetch)?;
        let table = server_admin_contract::AdminDataTable::from_frontend_path(
            server_admin_contract::AdminPagePathRef::from(pathname.as_str()),
        );
        Ok(Self {
            direction: params
                .get(constants_str::ADMIN_DIRECTION_QUERY_KEY)
                .map(server_admin_contract::AdminText::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?,
            filter_end: params
                .get(constants_str::ADMIN_FILTER_END_QUERY_KEY)
                .map(server_admin_contract::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?,
            filter_field: params
                .get(constants_str::ADMIN_FILTER_FIELD_QUERY_KEY)
                .map(server_admin_contract::AdminFilterField::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?,
            filter_operation: params
                .get(constants_str::ADMIN_FILTER_OPERATION_QUERY_KEY)
                .map(server_admin_contract::AdminFilterOperationKey::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?,
            filter_value: params
                .get(constants_str::ADMIN_FILTER_VALUE_QUERY_KEY)
                .map(server_admin_contract::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?,
            limit: params
                .get(constants_str::ADMIN_LIMIT_QUERY_KEY)
                .and_then(|value| value.parse::<u16>().ok())
                .and_then(|value| server_admin_contract::AdminPageLimit::try_from(value).ok())
                .unwrap_or_default(),
            offset: params
                .get(constants_str::ADMIN_OFFSET_QUERY_KEY)
                .and_then(|value| value.parse::<u32>().ok())
                .map_or_else(
                    server_admin_contract::AdminPageOffset::default,
                    server_admin_contract::AdminPageOffset::from,
                ),
            search: params
                .get(constants_str::ADMIN_SEARCH_QUERY_KEY)
                .map(server_admin_contract::AdminTableSearch::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            sort: params
                .get(constants_str::ADMIN_SORT_QUERY_KEY)
                .map(server_admin_contract::AdminTableSortKey::try_from)
                .transpose()
                .map_err(|_error| crate::app::state::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            table,
        })
    }
}
