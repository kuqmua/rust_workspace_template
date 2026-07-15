#![allow(
    unreachable_pub,
    reason = "Leptos component expansion and prelude visibility are confined by the private parent module"
)]
pub use leptos::prelude::*;
#[component]
pub(super) fn Shell(
    auth: server_admin_contract::AuthenticatedAdmin,
    client: super::AdminApiClient,
) -> impl IntoView {
    let auth = Some(auth);
    let loader = super::PageLoader::new();
    let nav = server_admin_contract::AdminPage::ALL;
    let initial_path = super::path();
    let current_path = RwSignal::new(if initial_path == str_constants::ADMIN_PAGE_PATHS_ROOT {
        let target = nav
            .iter()
            .find(|item| has_page_permission(&auth, **item))
            .copied()
            .unwrap_or(server_admin_contract::AdminPage::Version);
        let target_path = target.path().as_ref().to_owned();
        super::replace_path(target_path.as_str());
        target_path
    } else {
        initial_path
    });
    super::load(client.clone(), loader);
    if let Some(window) = web_sys::window() {
        let client_for_history = client.clone();
        let history_listener =
            wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |_event| {
                current_path.set(super::path());
                super::load(client_for_history.clone(), loader);
            });
        let history_listener_value = history_listener.into_js_value();
        window.set_onpopstate(Some(wasm_bindgen::JsCast::unchecked_ref(
            &history_listener_value,
        )));
    }
    let client_for_sign_out = client.clone();
    view! {
        <div class="app-shell"><header class="topbar"><a class="brand" href=str_constants::ADMIN_PAGE_PATHS_ROOT><span class="brand-mark">"A"</span><span><strong>"Admin"</strong><small>"Control center"</small></span></a><nav aria-label="Admin sections">
        {nav.into_iter().filter(|item| has_page_permission(&auth, *item)).map(|item| { let item_path = item.path().as_ref().to_owned(); let item_path_for_click = item_path.clone(); let item_title = item.title().as_ref().to_owned(); let client_for_nav = client.clone(); view! { <a class:active=move || current_path.get() == item_path href=item_path_for_click.clone() on:click=move |event| { event.prevent_default(); super::push_path(item_path_for_click.as_str()); current_path.set(item_path_for_click.clone()); super::load(client_for_nav.clone(), loader); }><span class="nav-dot"></span>{item_title}</a> } }).collect_view()}
        </nav><div class="profile"><div class="avatar">{auth.as_ref().and_then(|value| value.display_name().as_ref().chars().next()).unwrap_or('A').to_string()}</div><div><strong>{auth.as_ref().map(|value| value.display_name().to_string())}</strong><small>"Administrator"</small></div><button class="icon-button" title="Sign out" aria-label="Sign out" on:click=move |event| { event.prevent_default(); let client = client_for_sign_out.clone(); leptos::task::spawn_local(async move { if client.send(server_admin_contract::AdminRoute::SignOut).await.is_ok() { super::redirect(str_constants::ADMIN_PAGE_PATHS_SIGN_IN); } }); }>"Exit"</button></div></header>
        <main class="content"><PageView loader client auth /></main></div>
    }
}
#[component]
fn PageView(
    loader: super::PageLoader,
    client: super::AdminApiClient,
    auth: Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    move || {
        match loader.page().get() {
        super::Page::Loading => super::tables::loading().into_any(),
        super::Page::Error(value) => super::tables::error(value).into_any(),
        super::Page::Text(value) => view! { <section><div class="page-heading"><div><p class="eyebrow">"System"</p><h1>"Runtime information"</h1></div></div><div class="code-card"><pre>{value.to_string()}</pre></div></section> }.into_any(),
        super::Page::OpenApi(value) => view! { <section><div class="page-heading"><div><p class="eyebrow">"Developer tools"</p><h1>"OpenAPI document"</h1></div></div><div class="code-card api-document"><pre id="openapi">{value.to_string()}</pre></div></section> }.into_any(),
        super::Page::Users(values) => super::tables::users_view(values, client.clone(), loader, &auth).into_any(),
        super::Page::Roles(values) => super::tables::roles_view(values, client.clone(), loader, &auth).into_any(),
        super::Page::Permissions(values) => super::tables::permissions_view(values).into_any(),
        super::Page::Audit(values) => super::tables::audit_view(values).into_any(),
        super::Page::Settings(value) => super::forms::settings_view(value, client.clone(), loader, &auth).into_any(),
    }
    }
}
pub(super) fn has_page_permission(
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
    page: server_admin_contract::AdminPage,
) -> bool {
    has_authentication(auth, page.authentication())
}
pub(super) fn has_route_permission(
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
    route: server_admin_contract::AdminRoute,
) -> bool {
    has_authentication(auth, route.contract().authentication())
}
fn has_authentication(
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
    authentication: frontend_contract::AuthenticationRequirement,
) -> bool {
    match authentication {
        frontend_contract::AuthenticationRequirement::Authenticated => auth.is_some(),
        frontend_contract::AuthenticationRequirement::Permission(permission) => {
            auth.as_ref().is_some_and(|value| {
                value
                    .permissions()
                    .iter()
                    .any(|item| item.as_ref() == permission.as_ref())
            })
        }
        frontend_contract::AuthenticationRequirement::Public => true,
    }
}
