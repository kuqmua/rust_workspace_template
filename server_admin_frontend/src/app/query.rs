#[derive(Clone, Debug, Default)]
pub(in crate::app) struct AdminCsrQuery {
    pub(in crate::app) direction: Option<server_admin_contract::AdminText>,
    pub(in crate::app) filter_end: Option<server_admin_contract::AdminFilterValue>,
    pub(in crate::app) filter_field: Option<server_admin_contract::AdminFilterField>,
    pub(in crate::app) filter_operation: Option<server_admin_contract::AdminFilterOperationKey>,
    pub(in crate::app) filter_value: Option<server_admin_contract::AdminFilterValue>,
    pub(in crate::app) limit: server_admin_contract::AdminPageLimit,
    pub(in crate::app) offset: server_admin_contract::AdminPageOffset,
    pub(in crate::app) search: server_admin_contract::AdminTableSearch,
    pub(in crate::app) sort: server_admin_contract::AdminTableSortKey,
    pub(in crate::app) table: Option<server_admin_contract::AdminDataTable>,
}

impl AdminCsrQuery {
    pub(in crate::app) fn api_url(
        &self,
    ) -> Result<Option<super::http::AdminCsrApiUrl>, super::state::AdminTableLoadError> {
        let Some(table) = self.table else {
            return Ok(None);
        };
        let search = web_sys::window()
            .ok_or(super::state::AdminTableLoadError::Fetch)?
            .location()
            .search()
            .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
        super::http::admin_api_url_with_suffix(
            server_admin_contract::AdminRoute::DataTable(table),
            super::http::AdminCsrApiUrlSuffixRef::from(search.as_str()),
        )
        .map(Some)
    }

    pub(in crate::app) fn from_location() -> Result<Self, super::state::AdminTableLoadError> {
        let window = web_sys::window().ok_or(super::state::AdminTableLoadError::Fetch)?;
        let search = window
            .location()
            .search()
            .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
        let params = web_sys::UrlSearchParams::new_with_str(&search)
            .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
        let pathname = window
            .location()
            .pathname()
            .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
        let table = server_admin_contract::AdminDataTable::from_frontend_path(
            server_admin_contract::AdminPagePathRef::from(pathname.as_str()),
        );
        Ok(Self {
            direction: params
                .get(str_constants::ADMIN_DIRECTION_QUERY_KEY)
                .map(server_admin_contract::AdminText::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?,
            filter_end: params
                .get(str_constants::ADMIN_FILTER_END_QUERY_KEY)
                .map(server_admin_contract::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?,
            filter_field: params
                .get(str_constants::ADMIN_FILTER_FIELD_QUERY_KEY)
                .map(server_admin_contract::AdminFilterField::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?,
            filter_operation: params
                .get(str_constants::ADMIN_FILTER_OPERATION_QUERY_KEY)
                .map(server_admin_contract::AdminFilterOperationKey::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?,
            filter_value: params
                .get(str_constants::ADMIN_FILTER_VALUE_QUERY_KEY)
                .map(server_admin_contract::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?,
            limit: params
                .get(str_constants::ADMIN_LIMIT_QUERY_KEY)
                .and_then(|value| value.parse::<u16>().ok())
                .and_then(|value| server_admin_contract::AdminPageLimit::try_from(value).ok())
                .unwrap_or_default(),
            offset: params
                .get(str_constants::ADMIN_OFFSET_QUERY_KEY)
                .and_then(|value| value.parse::<u32>().ok())
                .map_or_else(
                    server_admin_contract::AdminPageOffset::default,
                    server_admin_contract::AdminPageOffset::from,
                ),
            search: params
                .get(str_constants::ADMIN_SEARCH_QUERY_KEY)
                .map(server_admin_contract::AdminTableSearch::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            sort: params
                .get(str_constants::ADMIN_SORT_QUERY_KEY)
                .map(server_admin_contract::AdminTableSortKey::try_from)
                .transpose()
                .map_err(|_error| super::state::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            table,
        })
    }
}

pub(in crate::app) fn csr_page_from_location()
-> Result<server_admin_contract::AdminPage, super::state::AdminTableLoadError> {
    let pathname = web_sys::window()
        .ok_or(super::state::AdminTableLoadError::Fetch)?
        .location()
        .pathname()
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    let path = server_admin_contract::AdminPagePathRef::from(pathname.as_str());
    let page = match server_admin_contract::AdminPage::from_path(path) {
        Some(page) => page,
        None if server_admin_contract::AdminDataTable::from_frontend_path(path).is_some() => {
            server_admin_contract::AdminPage::Tables
        }
        None => return Err(super::state::AdminTableLoadError::Query),
    };
    if bool::from(page.supports_csr()) {
        Ok(page)
    } else {
        Err(super::state::AdminTableLoadError::Query)
    }
}
