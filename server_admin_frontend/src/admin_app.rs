use leptos::prelude::{ClassAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos root component visibility is required by the app entry point"
)]
pub(crate) fn AdminApp() -> impl leptos::prelude::IntoView {
    let query_result = super::admin_csr_query::AdminCsrQuery::from_location();
    let page_result = super::admin_csr_query::csr_page_from_location::csr_page_from_location();
    let initial_state = match (&page_result, &query_result) {
        (Ok(_page), Ok(_query)) => super::state::admin_load_state::AdminLoadState::Loading,
        (Err(error), _) | (_, Err(error)) => {
            super::state::admin_load_state::AdminLoadState::Error(error.clone())
        }
    };
    let state = leptos::prelude::RwSignal::new(initial_state);
    if let (Ok(page), Ok(query)) = (page_result, query_result.clone()) {
        wasm_bindgen_futures::spawn_local(async move {
            let next_state = match super::fetch_page::fetch_page(page, &query).await {
                Ok(value) => value,
                Err(error) => super::state::admin_load_state::AdminLoadState::Error(error),
            };
            leptos::prelude::Set::set(&state, next_state);
        });
    }
    leptos::view! {
        <div class="app-shell">
        {move || {
            let current_state = leptos::prelude::Get::get(&state);
            let admin = current_state.admin().cloned();
            let content = match current_state {
                super::state::admin_load_state::AdminLoadState::Empty(_admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_empty::AdminEmpty>"Choose a table."</crate::admin_empty::AdminEmpty> }),
                super::state::admin_load_state::AdminLoadState::Error(error) => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_alert::AdminAlert>{error.to_string()}</crate::admin_alert::AdminAlert> }),
                super::state::admin_load_state::AdminLoadState::Loading => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_spinner::AdminSpinner /> }),
                super::state::admin_load_state::AdminLoadState::Permissions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_permissions_view::AdminPermissionsView page=page query=query_result.clone().unwrap_or_default() /> }),
                super::state::admin_load_state::AdminLoadState::Profile(admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_profile_view::AdminProfileView admin=admin /> }),
                super::state::admin_load_state::AdminLoadState::Roles(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_roles_view::AdminRolesView admin=admin page=page query=query_result.clone().unwrap_or_default() /> }),
                super::state::admin_load_state::AdminLoadState::Sessions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_sessions_view::AdminSessionsView page=page /> }),
                super::state::admin_load_state::AdminLoadState::Settings(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_settings_view::AdminSettingsView admin=admin page=page /> }),
                super::state::admin_load_state::AdminLoadState::Table(_admin, view) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_data_grid::AdminDataGrid view=view query=query_result.clone().unwrap_or_default() /> }),
                super::state::admin_load_state::AdminLoadState::Users(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::admin_users_view::AdminUsersView admin=admin page=page query=query_result.clone().unwrap_or_default() /> }),
            };
            leptos::view! { <super::csr_admin_nav::CsrAdminNav admin=admin /><main class="main-content"><div class="page-frame">{content}</div></main> }
        }}</div>
    }
}
