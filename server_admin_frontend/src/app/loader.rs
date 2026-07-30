#[allow(
    clippy::future_not_send,
    reason = "browser page loads run exclusively on wasm_bindgen_futures::spawn_local"
)]
pub(in crate::app) async fn fetch_page(
    page: server_admin_contract::AdminPage,
    query: &super::query::AdminCsrQuery,
) -> Result<super::state::AdminLoadState, super::state::AdminTableLoadError> {
    let search = web_sys::window()
        .ok_or(super::state::AdminTableLoadError::Fetch)?
        .location()
        .search()
        .map_err(|_error| super::state::AdminTableLoadError::Fetch)?;
    let me_url = super::http::admin_api_url(server_admin_contract::AdminRoute::Me)?;
    let admin =
        super::http::fetch_json::<server_admin_contract::AuthenticatedAdmin>(&me_url).await?;
    let route = match page {
        server_admin_contract::AdminPage::Profile => {
            return Ok(super::state::AdminLoadState::Profile(admin));
        }
        server_admin_contract::AdminPage::Tables => {
            let Some(url) = query.api_url()? else {
                return Ok(super::state::AdminLoadState::Empty(admin));
            };
            return super::http::fetch_json(&url)
                .await
                .map(|value| super::state::AdminLoadState::Table(admin, value));
        }
        server_admin_contract::AdminPage::Permissions
        | server_admin_contract::AdminPage::Roles
        | server_admin_contract::AdminPage::Sessions
        | server_admin_contract::AdminPage::Settings
        | server_admin_contract::AdminPage::Users => page.spec().route(),
        server_admin_contract::AdminPage::Metrics
        | server_admin_contract::AdminPage::OpenApi
        | server_admin_contract::AdminPage::Version => {
            return Err(super::state::AdminTableLoadError::Query);
        }
    };
    let suffix = if bool::from(page.uses_table_query()) {
        search
    } else {
        String::new()
    };
    let url = super::http::admin_api_url_with_suffix(
        route,
        super::http::AdminCsrApiUrlSuffixRef::from(suffix.as_str()),
    )?;
    match page {
        server_admin_contract::AdminPage::Permissions => super::http::fetch_json(&url)
            .await
            .map(|value| super::state::AdminLoadState::Permissions(admin, value)),
        server_admin_contract::AdminPage::Profile => {
            Ok(super::state::AdminLoadState::Profile(admin))
        }
        server_admin_contract::AdminPage::Roles => super::http::fetch_json(&url)
            .await
            .map(|value| super::state::AdminLoadState::Roles(admin, value)),
        server_admin_contract::AdminPage::Sessions => super::http::fetch_json(&url)
            .await
            .map(|value| super::state::AdminLoadState::Sessions(admin, value)),
        server_admin_contract::AdminPage::Settings => super::http::fetch_json(&url)
            .await
            .map(|value| super::state::AdminLoadState::Settings(admin, value)),
        server_admin_contract::AdminPage::Tables => Ok(super::state::AdminLoadState::Empty(admin)),
        server_admin_contract::AdminPage::Users => super::http::fetch_json(&url)
            .await
            .map(|value| super::state::AdminLoadState::Users(admin, value)),
        server_admin_contract::AdminPage::Metrics
        | server_admin_contract::AdminPage::OpenApi
        | server_admin_contract::AdminPage::Version => {
            Err(super::state::AdminTableLoadError::Query)
        }
    }
}
