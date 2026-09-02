#[allow(
    clippy::future_not_send,
    reason = "browser admin_page loads run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(crate) async fn fetch_page(
    admin_page: server_admin_contract::admin_page::AdminPage,
    admin_csr_query: &super::admin_csr_query::AdminCsrQuery,
) -> Result<
    crate::admin_load_state::AdminLoadState,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let search = web_sys::window()
        .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?
        .location()
        .search()
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
    let me_url =
        crate::admin_api_url::admin_api_url(server_admin_contract::admin_route::AdminRoute::Me)?;
    let admin = crate::fetch_json::fetch_json::<
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    >(&me_url)
    .await?;
    let route = match admin_page {
        server_admin_contract::admin_page::AdminPage::Profile => {
            return Ok(crate::admin_load_state::AdminLoadState::Profile(admin));
        }
        server_admin_contract::admin_page::AdminPage::Tables => {
            let Some(table) = admin_csr_query.table() else {
                return Ok(crate::admin_load_state::AdminLoadState::Empty(admin));
            };
            let table_search = web_sys::window()
                .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?
                .location()
                .search()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
            let url = crate::admin_api_url_with_suffix::admin_api_url_with_suffix(
                server_admin_contract::admin_route::AdminRoute::DataTable(table),
                crate::admin_csr_api_url_suffix_ref::AdminCsrApiUrlSuffixRef::from(
                    table_search.as_str(),
                ),
            )?;
            return crate::fetch_json::fetch_json(&url)
                .await
                .map(|value| crate::admin_load_state::AdminLoadState::Table(admin, value));
        }
        server_admin_contract::admin_page::AdminPage::Permissions
        | server_admin_contract::admin_page::AdminPage::Roles
        | server_admin_contract::admin_page::AdminPage::Sessions
        | server_admin_contract::admin_page::AdminPage::Settings
        | server_admin_contract::admin_page::AdminPage::Users => admin_page.spec().route(),
        server_admin_contract::admin_page::AdminPage::Metrics
        | server_admin_contract::admin_page::AdminPage::OpenApi
        | server_admin_contract::admin_page::AdminPage::Version => {
            return Err(crate::admin_table_load_error::AdminTableLoadError::Query);
        }
    };
    let suffix = if bool::from(admin_page.uses_table_query()) {
        search
    } else {
        String::new()
    };
    let url = crate::admin_api_url_with_suffix::admin_api_url_with_suffix(
        route,
        crate::admin_csr_api_url_suffix_ref::AdminCsrApiUrlSuffixRef::from(suffix.as_str()),
    )?;
    match admin_page {
        server_admin_contract::admin_page::AdminPage::Permissions => {
            crate::fetch_json::fetch_json(&url)
                .await
                .map(|value| crate::admin_load_state::AdminLoadState::Permissions(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Profile => {
            Ok(crate::admin_load_state::AdminLoadState::Profile(admin))
        }
        server_admin_contract::admin_page::AdminPage::Roles => crate::fetch_json::fetch_json(&url)
            .await
            .map(|value| crate::admin_load_state::AdminLoadState::Roles(admin, value)),
        server_admin_contract::admin_page::AdminPage::Sessions => {
            crate::fetch_json::fetch_json(&url)
                .await
                .map(|value| crate::admin_load_state::AdminLoadState::Sessions(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Settings => {
            crate::fetch_json::fetch_json(&url)
                .await
                .map(|value| crate::admin_load_state::AdminLoadState::Settings(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Tables => {
            Ok(crate::admin_load_state::AdminLoadState::Empty(admin))
        }
        server_admin_contract::admin_page::AdminPage::Users => crate::fetch_json::fetch_json(&url)
            .await
            .map(|value| crate::admin_load_state::AdminLoadState::Users(admin, value)),
        server_admin_contract::admin_page::AdminPage::Metrics
        | server_admin_contract::admin_page::AdminPage::OpenApi
        | server_admin_contract::admin_page::AdminPage::Version => {
            Err(crate::admin_table_load_error::AdminTableLoadError::Query)
        }
    }
}
