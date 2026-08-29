#[allow(
    clippy::future_not_send,
    reason = "browser page loads run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(crate) async fn fetch_page(
    page: server_admin_contract::admin_page::AdminPage,
    query: &super::admin_csr_query::AdminCsrQuery,
) -> Result<
    super::state::admin_load_state::AdminLoadState,
    super::state::admin_table_load_error::AdminTableLoadError,
> {
    let search = web_sys::window()
        .ok_or(super::state::admin_table_load_error::AdminTableLoadError::Fetch)?
        .location()
        .search()
        .map_err(|_error| super::state::admin_table_load_error::AdminTableLoadError::Fetch)?;
    let me_url =
        super::http::url::admin_api_url(server_admin_contract::admin_route::AdminRoute::Me)?;
    let admin = super::http::fetch_json::fetch_json::<
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    >(&me_url)
    .await?;
    let route = match page {
        server_admin_contract::admin_page::AdminPage::Profile => {
            return Ok(super::state::admin_load_state::AdminLoadState::Profile(
                admin,
            ));
        }
        server_admin_contract::admin_page::AdminPage::Tables => {
            let Some(table) = query.table else {
                return Ok(super::state::admin_load_state::AdminLoadState::Empty(admin));
            };
            let table_search = web_sys::window()
                .ok_or(super::state::admin_table_load_error::AdminTableLoadError::Fetch)?
                .location()
                .search()
                .map_err(|_error| {
                    super::state::admin_table_load_error::AdminTableLoadError::Fetch
                })?;
            let url = super::http::url::admin_api_url_with_suffix(
                server_admin_contract::admin_route::AdminRoute::DataTable(table),
                super::http::url::AdminCsrApiUrlSuffixRef::from(table_search.as_str()),
            )?;
            return super::http::fetch_json::fetch_json(&url)
                .await
                .map(|value| super::state::admin_load_state::AdminLoadState::Table(admin, value));
        }
        server_admin_contract::admin_page::AdminPage::Permissions
        | server_admin_contract::admin_page::AdminPage::Roles
        | server_admin_contract::admin_page::AdminPage::Sessions
        | server_admin_contract::admin_page::AdminPage::Settings
        | server_admin_contract::admin_page::AdminPage::Users => page.spec().route(),
        server_admin_contract::admin_page::AdminPage::Metrics
        | server_admin_contract::admin_page::AdminPage::OpenApi
        | server_admin_contract::admin_page::AdminPage::Version => {
            return Err(super::state::admin_table_load_error::AdminTableLoadError::Query);
        }
    };
    let suffix = if bool::from(page.uses_table_query()) {
        search
    } else {
        String::new()
    };
    let url = super::http::url::admin_api_url_with_suffix(
        route,
        super::http::url::AdminCsrApiUrlSuffixRef::from(suffix.as_str()),
    )?;
    match page {
        server_admin_contract::admin_page::AdminPage::Permissions => {
            super::http::fetch_json::fetch_json(&url)
                .await
                .map(|value| {
                    super::state::admin_load_state::AdminLoadState::Permissions(admin, value)
                })
        }
        server_admin_contract::admin_page::AdminPage::Profile => Ok(
            super::state::admin_load_state::AdminLoadState::Profile(admin),
        ),
        server_admin_contract::admin_page::AdminPage::Roles => {
            super::http::fetch_json::fetch_json(&url)
                .await
                .map(|value| super::state::admin_load_state::AdminLoadState::Roles(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Sessions => {
            super::http::fetch_json::fetch_json(&url)
                .await
                .map(|value| super::state::admin_load_state::AdminLoadState::Sessions(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Settings => {
            super::http::fetch_json::fetch_json(&url)
                .await
                .map(|value| super::state::admin_load_state::AdminLoadState::Settings(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Tables => {
            Ok(super::state::admin_load_state::AdminLoadState::Empty(admin))
        }
        server_admin_contract::admin_page::AdminPage::Users => {
            super::http::fetch_json::fetch_json(&url)
                .await
                .map(|value| super::state::admin_load_state::AdminLoadState::Users(admin, value))
        }
        server_admin_contract::admin_page::AdminPage::Metrics
        | server_admin_contract::admin_page::AdminPage::OpenApi
        | server_admin_contract::admin_page::AdminPage::Version => {
            Err(super::state::admin_table_load_error::AdminTableLoadError::Query)
        }
    }
}
