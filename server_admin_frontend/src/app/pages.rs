#![allow(
    unreachable_pub,
    reason = "Leptos component expansion and prelude visibility are confined by the private parent module"
)]
pub use leptos::prelude::*;
#[component]
pub(super) fn Shell(
    auth: server_admin_contract::AuthenticatedAdmin,
    client: super::AdminApiClient,
    branding: Option<server_admin_contract::AdminBrandingView>,
) -> impl IntoView {
    let auth = Some(auth);
    let loader = super::PageLoader::new();
    let nav = server_admin_contract::AdminPage::all().collect::<Vec<_>>();
    let initial_path = super::path();
    let current_path = RwSignal::new(
        if initial_path == server_admin_contract::AdminFrontendPath::Root.get() {
            let configured_target = branding.as_ref().and_then(|value| {
                server_admin_contract::AdminPage::from_path(
                    server_admin_contract::AdminPagePathRef::from(
                        value.default_admin_route().as_ref(),
                    ),
                )
            });
            let target = configured_target
                .filter(|item| has_page_permission(&auth, *item))
                .or_else(|| {
                    nav.iter()
                        .find(|item| has_page_permission(&auth, **item))
                        .copied()
                })
                .unwrap_or(server_admin_contract::AdminPage::Version);
            let target_path = target.path().as_ref().to_owned();
            super::replace_path(target_path.as_str());
            target_path
        } else {
            initial_path
        },
    );
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
    let site_name = branding.as_ref().map_or_else(
        || str_constants::ADMIN.to_owned(),
        |value| value.site_name().as_ref().to_owned(),
    );
    let logo = branding.and_then(|value| value.main_logo().cloned());
    view! {
        <div class="app-shell"><header class="topbar"><a class="brand" href=server_admin_contract::AdminFrontendPath::Root.get()>{logo.map_or_else(|| view! { <span class="brand-mark">"A"</span> }.into_any(), |value| view! { <img class="brand-logo" src=value.as_ref().to_owned() alt="" /> }.into_any())}<span><strong>{site_name}</strong><small>"Control center"</small></span></a><nav aria-label="Admin sections">
        {nav.into_iter().filter(|item| has_page_permission(&auth, *item)).map(|item| { let item_path = item.path().as_ref().to_owned(); let item_path_for_click = item_path.clone(); let item_title = item.title().as_ref().to_owned(); let client_for_nav = client.clone(); view! { <a class:active=move || current_path.get() == item_path href=item_path_for_click.clone() on:click=move |event| { event.prevent_default(); super::push_path(item_path_for_click.as_str()); current_path.set(item_path_for_click.clone()); super::load(client_for_nav.clone(), loader); }><span class="nav-dot"></span>{item_title}</a> } }).collect_view()}
        </nav><div class="profile"><div class="avatar">{auth.as_ref().and_then(|value| value.display_name().as_ref().chars().next()).unwrap_or('A').to_string()}</div><div><strong>{auth.as_ref().map(|value| value.display_name().to_string())}</strong><small>"Administrator"</small></div><button class="icon-button" title="Sign out" aria-label="Sign out" on:click=move |event| { event.prevent_default(); let client = client_for_sign_out.clone(); leptos::task::spawn_local(async move { if client.send(server_admin_contract::AdminRoute::SignOut).await.is_ok() { super::redirect(server_admin_contract::AdminFrontendPath::SignIn.get()); } }); }>"Exit"</button></div></header>
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
        let content = match loader.page().get() {
        super::Page::Loading => super::tables::loading().into_any(),
        super::Page::Dashboard(value) => dashboard_view(&value).into_any(),
        super::Page::Profile(value) => super::forms::profile_view(value, client.clone(), loader, &auth).into_any(),
        super::Page::Error(value) => super::tables::error(value).into_any(),
        super::Page::Text(value) => view! { <section><div class="page-heading"><div><p class="eyebrow">"System"</p><h1>"Runtime information"</h1></div></div><div class="code-card"><pre>{value.to_string()}</pre></div></section> }.into_any(),
        super::Page::OpenApi(value) => view! { <section><div class="page-heading"><div><p class="eyebrow">"Developer tools"</p><h1>"OpenAPI document"</h1></div></div><div class="code-card api-document"><pre id="openapi">{value.to_string()}</pre></div></section> }.into_any(),
        super::Page::Users(values, roles, total) => super::tables::users_view(values, roles, total, client.clone(), loader, &auth).into_any(),
        super::Page::Roles(values, permissions, total) => super::tables::roles_view(values, permissions, total, client.clone(), loader, &auth).into_any(),
        super::Page::Permissions(values, total) => super::tables::permissions_view(values, total, client.clone(), loader).into_any(),
        super::Page::Audit(values, next_cursor) => super::tables::audit_view(values, next_cursor, client.clone(), loader, &auth).into_any(),
        super::Page::Settings(value) => super::forms::settings_view(value, client.clone(), loader, &auth).into_any(),
        super::Page::Sessions(values) => super::tables::sessions_view(values, client.clone(), loader).into_any(),
        };
        let notice = loader.notice.get().map(|value| view! {
            <div class="alert success page-alert" role="status"><span>{value.to_string()}</span><button aria-label="Dismiss notification" on:click=move |_| loader.notice.set(None)>"Dismiss"</button></div>
        });
        view! { {notice}{content} }
    }
}
fn dashboard_view(value: &server_admin_contract::AdminDashboardView) -> impl IntoView {
    let cleanup_status = value.last_cleanup().map_or_else(
        || view! { <article class="summary-card"><span>"Cleanup job"</span><strong>"No successful run recorded"</strong></article> }.into_any(),
        |status| view! { <article class="summary-card"><span>"Last cleanup"</span><strong>{status.last_success_at().to_string()}</strong><small>{format!("{} rows deleted", status.deleted_rows())}</small></article> }.into_any(),
    );
    let recent_changes = value
        .recent_changes()
        .iter()
        .map(|event| {
            view! { <li><strong>{event.action().to_string()}</strong><span>{format!("{} - {}", event.resource(), event.created_at())}</span></li> }
        })
        .collect_view();
    view! { <section><div class="page-heading"><div><p class="eyebrow">"Operations"</p><h1>"Dashboard"</h1></div></div><div class="dashboard-grid"><article class="summary-card"><span>"Database"</span><strong>{if bool::from(value.database_healthy()) { "Healthy" } else { "Unavailable" }}</strong></article><article class="summary-card"><span>"Version"</span><strong>{value.version().to_string()}</strong></article><article class="summary-card"><span>"Uptime"</span><strong>{format!("{} seconds", value.uptime_seconds())}</strong></article><article class="summary-card"><span>"Active sessions"</span><strong>{value.active_sessions().to_string()}</strong></article><article class="summary-card"><span>"Failed sign-ins (24h)"</span><strong>{value.failed_sign_ins_24h().to_string()}</strong></article>{cleanup_status}</div><article class="recent-changes"><h2>"Recent administrative changes"</h2><ul>{recent_changes}</ul><a href=server_admin_contract::AdminFrontendPath::Audit.get()>"Open audit log"</a></article></section> }
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
