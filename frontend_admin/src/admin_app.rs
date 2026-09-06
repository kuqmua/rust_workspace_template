#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos root component visibility is required by the app entry point"
)]
pub(crate) fn AdminApp() -> impl leptos::prelude::IntoView {
    let password_change_required = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(constants_str::ADMIN_CSR_ROOT_ID))
        .is_some_and(|element| {
            element.has_attribute(constants_str::ADMIN_PASSWORD_CHANGE_REQUIRED_ATTRIBUTE)
        });
    #[allow(
        clippy::future_not_send,
        reason = "browser admin_page loads run exclusively on wasm_bindgen_futures::spawn_local"
    )]
    let fetch_page = async |admin_page: server_admin_contract::admin_page::AdminPage,
                            admin_csr_query: &super::admin_csr_query::AdminCsrQuery|
           -> Result<
        crate::admin_load_state::AdminLoadState,
        crate::admin_table_load_error::AdminTableLoadError,
    > {
        let search = web_sys::window()
            .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?
            .location()
            .search()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let me_url = crate::admin_api_url::admin_api_url(
            server_admin_contract::admin_route::AdminRoute::Me,
        )?;
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
            server_admin_contract::admin_page::AdminPage::Roles => {
                crate::fetch_json::fetch_json(&url)
                    .await
                    .map(|value| crate::admin_load_state::AdminLoadState::Roles(admin, value))
            }
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
            server_admin_contract::admin_page::AdminPage::Users => {
                crate::fetch_json::fetch_json(&url)
                    .await
                    .map(|value| crate::admin_load_state::AdminLoadState::Users(admin, value))
            }
            server_admin_contract::admin_page::AdminPage::Metrics
            | server_admin_contract::admin_page::AdminPage::OpenApi
            | server_admin_contract::admin_page::AdminPage::Version => {
                Err(crate::admin_table_load_error::AdminTableLoadError::Query)
            }
        }
    };

    let csr_page_from_location = || {
        let pathname = web_sys::window()
            .ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?
            .location()
            .pathname()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let path =
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str());
        let page = match server_admin_contract::admin_page::AdminPage::from_path(path) {
            Some(page) => page,
            None
                if server_admin_contract::admin_data_table::AdminDataTable::from_frontend_path(
                    path,
                )
                .is_some() =>
            {
                server_admin_contract::admin_page::AdminPage::Tables
            }
            None => return Err(crate::admin_table_load_error::AdminTableLoadError::Query),
        };
        if bool::from(page.supports_csr()) {
            Ok(page)
        } else {
            Err(crate::admin_table_load_error::AdminTableLoadError::Query)
        }
    };

    let query_result = super::admin_csr_query::AdminCsrQuery::from_location();
    let page_result = csr_page_from_location();
    let initial_state = match (&page_result, &query_result) {
        (Ok(_page), Ok(_query)) => crate::admin_load_state::AdminLoadState::Loading,
        (Err(error), _) | (_, Err(error)) => {
            crate::admin_load_state::AdminLoadState::Error(error.clone())
        }
    };
    let state = leptos::prelude::RwSignal::new(initial_state);
    if let (Ok(page), Ok(query)) = (page_result, query_result.clone()) {
        wasm_bindgen_futures::spawn_local(async move {
            let next_state = match fetch_page(page, &query).await {
                Ok(value) => value,
                Err(error) => crate::admin_load_state::AdminLoadState::Error(error),
            };
            leptos::prelude::Set::set(&state, next_state);
        });
    }
    leptos::view! {
        <div class="app-shell">
        {move || {
            let current_state = leptos::prelude::Get::get(&state);
            let navigation_admin = current_state.admin().cloned();
            let content = match current_state {
                crate::admin_load_state::AdminLoadState::Empty(_admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_empty::AdminEmpty>{constants_str::ADMIN_UI_CHOOSE_A_TABLE}</crate::admin_empty::AdminEmpty> }),
                crate::admin_load_state::AdminLoadState::Error(error) => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_alert::AdminAlert>{error.to_string()}</crate::admin_alert::AdminAlert> }),
                crate::admin_load_state::AdminLoadState::Loading => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_spinner::AdminSpinner /> }),
                crate::admin_load_state::AdminLoadState::Permissions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_permissions_view::AdminPermissionsView admin_permissions_page=page admin_csr_query=query_result.clone().unwrap_or_default() /> }),
                crate::admin_load_state::AdminLoadState::Profile(admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_profile_view::AdminProfileView authenticated_admin=admin /> }),
                crate::admin_load_state::AdminLoadState::Roles(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_roles_view::AdminRolesView authenticated_admin=admin admin_roles_page=page admin_csr_query=query_result.clone().unwrap_or_default() /> }),
                crate::admin_load_state::AdminLoadState::Sessions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_sessions_view::AdminSessionsView admin_sessions_page=page /> }),
                crate::admin_load_state::AdminLoadState::Settings(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_settings_view::AdminSettingsView authenticated_admin=admin admin_settings_view=page /> }),
                crate::admin_load_state::AdminLoadState::Table(admin, view) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_data_grid::AdminDataGrid admin_bool=admin.has_permission(server_admin_contract::admin_permission::AdminPermission::AuditLogExport) admin_data_table_view=view admin_csr_query=query_result.clone().unwrap_or_default() /> }),
                crate::admin_load_state::AdminLoadState::Users(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_users_view::AdminUsersView authenticated_admin=admin admin_users_page=page admin_csr_query=query_result.clone().unwrap_or_default() /> }),
            };
            leptos::view! { <super::csr_admin_nav::CsrAdminNav option=navigation_admin admin_bool=server_admin_contract::admin_bool::AdminBool::from(password_change_required) /><main class="main-content"><div class="page-frame">{password_change_required.then(|| leptos::view! { <crate::admin_alert::AdminAlert>{constants_str::ADMIN_UI_CHANGE_YOUR_INITIAL_PASSWORD_TO_UNLOCK_ADMINISTRATOR_NAVIGATION}</crate::admin_alert::AdminAlert> })}{content}</div></main> }
        }}</div>
    }
}
