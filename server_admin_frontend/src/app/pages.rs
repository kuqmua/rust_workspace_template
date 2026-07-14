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
    let page = RwSignal::new(super::Page::Loading);
    let nav = server_admin_contract::AdminPage::ALL;
    if super::path() == "/admin" {
        let target = nav
            .iter()
            .find(|item| has_page_permission(&auth, **item))
            .copied()
            .unwrap_or(server_admin_contract::AdminPage::Version);
        super::redirect(target.path().as_ref());
    } else {
        super::load(client.clone(), page);
    }
    let client_for_sign_out = client.clone();
    let current_path = super::path();
    view! {
        <div class="app-shell"><header class="topbar"><a class="brand" href="/admin"><span class="brand-mark">"A"</span><span><strong>"Admin"</strong><small>"Control center"</small></span></a><nav aria-label="Admin sections">
        {nav.into_iter().filter(|item| has_page_permission(&auth, *item)).map(|item| { let item_path = item.path().as_ref().to_owned(); let item_title = item.title().as_ref().to_owned(); let active = current_path == item_path; view! { <a class:active=active href=item_path><span class="nav-dot"></span>{item_title}</a> } }).collect_view()}
        </nav><div class="profile"><div class="avatar">{auth.as_ref().and_then(|value| value.display_name().as_ref().chars().next()).unwrap_or('A').to_string()}</div><div><strong>{auth.as_ref().map(|value| value.display_name().to_string())}</strong><small>"Administrator"</small></div><button class="icon-button" title="Sign out" aria-label="Sign out" on:click=move |event| { event.prevent_default(); let client = client_for_sign_out.clone(); leptos::task::spawn_local(async move { if client.send(server_admin_contract::AdminRoute::SignOut).await.is_ok() { super::redirect("/admin/sign-in"); } }); }>"Exit"</button></div></header>
        <main class="content"><PageView page client auth /></main></div>
    }
}
#[component]
fn PageView(
    page: RwSignal<super::Page>,
    client: super::AdminApiClient,
    auth: Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    move || {
        match page.get() {
        super::Page::Loading => super::tables::loading().into_any(),
        super::Page::Error(value) => super::tables::error(value).into_any(),
        super::Page::Text(value) => view! { <section><div class="page-heading"><div><p class="eyebrow">"System"</p><h1>"Runtime information"</h1></div></div><div class="code-card"><pre>{value.to_string()}</pre></div></section> }.into_any(),
        super::Page::Users(values) => super::tables::users_view(values, client.clone(), page, &auth).into_any(),
        super::Page::Roles(values) => super::tables::roles_view(values, client.clone(), page, &auth).into_any(),
        super::Page::Permissions(values) => super::tables::permissions_view(values).into_any(),
        super::Page::Audit(values) => super::tables::audit_view(values).into_any(),
        super::Page::Settings(value) => super::forms::settings_view(value, client.clone(), page, &auth).into_any(),
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
