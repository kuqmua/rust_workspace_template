#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures remain on the browser thread"
)]
pub(crate) async fn fetch_rate_limit_read(
    admin_rate_limit_id: &server_admin_contract::admin_rate_limit_id::AdminRateLimitId,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let mut table_search = String::new();
    table_search.push('?');
    table_search.push_str(constants_str::ADMIN_FILTER_FIELD_QUERY_KEY);
    table_search.push('=');
    table_search.push_str(constants_str::SCOPE);
    table_search.push('&');
    table_search.push_str(constants_str::ADMIN_FILTER_OPERATION_QUERY_KEY);
    table_search.push('=');
    table_search.push_str(constants_str::ADMIN_FILTER_OPERATION_EQ);
    table_search.push('&');
    table_search.push_str(constants_str::ADMIN_FILTER_VALUE_QUERY_KEY);
    table_search.push('=');
    table_search.push_str(admin_rate_limit_id.scope().as_ref().as_str());
    let url = crate::admin_api_url_with_suffix::admin_api_url_with_suffix(
        server_admin_contract::admin_route::AdminRoute::RateLimitsTable,
        crate::admin_csr_api_url_suffix_ref::AdminCsrApiUrlSuffixRef::from(table_search.as_str()),
    )?;
    let view = crate::fetch_json::fetch_json::<
        server_admin_contract::admin_data_table_view::AdminDataTableView,
    >(&url)
    .await?;
    let subject_index = view
        .columns()
        .iter()
        .position(|column| column.name().as_ref() == constants_str::SUBJECT)
        .ok_or(crate::admin_table_load_error::AdminTableLoadError::Response)?;
    let items = view
        .items()
        .iter()
        .filter(|row| {
            row.values().get(subject_index).is_some_and(|subject| {
                subject.as_ref().as_str() == admin_rate_limit_id.subject().as_ref().as_str()
            })
        })
        .cloned()
        .collect::<Vec<_>>();
    let total = server_admin_contract::admin_page_total::AdminPageTotal::from(
        items
            .iter()
            .fold(0u64, |total, _item| total.saturating_add(1u64)),
    );
    Ok(
        server_admin_contract::admin_data_table_view::AdminDataTableView::new(
            server_admin_contract::admin_data_columns::AdminDataColumns::try_from(
                view.columns().to_vec(),
            )?,
            server_admin_contract::admin_data_rows::AdminDataRows::try_from(items)?,
            server_admin_contract::admin_data_table::AdminDataTable::RateLimits,
            total,
        ),
    )
}
