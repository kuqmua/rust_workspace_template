use leptos::prelude::{ClassAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos root component visibility is required by the app entry point"
)]
pub(in crate::domain_types::start) fn AdminApp() -> impl leptos::prelude::IntoView {
    let query_result = super::query::AdminCsrQuery::from_location();
    let page_result = super::query::page::csr_page_from_location();
    let initial_state = match (&page_result, &query_result) {
        (Ok(_page), Ok(_query)) => super::state::admin_load_state::AdminLoadState::Loading,
        (Err(error), _) | (_, Err(error)) => {
            super::state::admin_load_state::AdminLoadState::Error(error.clone())
        }
    };
    let state = leptos::prelude::RwSignal::new(initial_state);
    if let (Ok(page), Ok(query)) = (page_result, query_result.clone()) {
        wasm_bindgen_futures::spawn_local(async move {
            let next_state = match super::loader::fetch_page(page, &query).await {
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
                super::state::admin_load_state::AdminLoadState::Empty(_admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::domain_types::with_owner::admin_empty::AdminEmpty>"Choose a table."</crate::domain_types::with_owner::admin_empty::AdminEmpty> }),
                super::state::admin_load_state::AdminLoadState::Error(error) => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::domain_types::with_owner::alert::AdminAlert>{error.to_string()}</crate::domain_types::with_owner::alert::AdminAlert> }),
                super::state::admin_load_state::AdminLoadState::Loading => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::domain_types::with_owner::admin_spinner::AdminSpinner /> }),
                super::state::admin_load_state::AdminLoadState::Permissions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::permissions::AdminPermissionsView page=page query=query_result.clone().unwrap_or_default() /> }),
                super::state::admin_load_state::AdminLoadState::Profile(admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::profile::AdminProfileView admin=admin /> }),
                super::state::admin_load_state::AdminLoadState::Roles(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::roles::AdminRolesView admin=admin page=page query=query_result.clone().unwrap_or_default() /> }),
                super::state::admin_load_state::AdminLoadState::Sessions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::sessions::AdminSessionsView page=page /> }),
                super::state::admin_load_state::AdminLoadState::Settings(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::settings::AdminSettingsView admin=admin page=page /> }),
                super::state::admin_load_state::AdminLoadState::Table(_admin, view) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::data_grid::AdminDataGrid view=view query=query_result.clone().unwrap_or_default() /> }),
                super::state::admin_load_state::AdminLoadState::Users(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <super::users::AdminUsersView admin=admin page=page query=query_result.clone().unwrap_or_default() /> }),
            };
            leptos::view! { <super::navigation::AdminNav admin=admin /><main class="main-content"><div class="page-frame">{content}</div></main> }
        }}</div>
    }
}
