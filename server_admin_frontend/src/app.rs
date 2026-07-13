#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::empty_structs_with_brackets,
    clippy::future_not_send,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::no_effect_underscore_binding,
    clippy::non_ascii_literal,
    clippy::option_if_let_else,
    clippy::redundant_clone,
    clippy::ref_option,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::single_call_fn,
    clippy::single_match_else,
    clippy::unused_async,
    clippy::unused_async_trait_impl,
    clippy::unused_trait_names
)] // Leptos component expansion and single-threaded browser futures intentionally conflict with server-oriented workspace lints
pub use leptos::prelude::*;
pub use wasm_bindgen::JsCast;
const API: &str = "/api/v1/admin";
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
struct Text(String);
impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
struct Id(i64);
impl TryFrom<&str> for Id {
    type Error = std::num::ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse().map(Self)
    }
}
impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Debug, serde::Deserialize)]
struct AuthenticatedAdmin {
    display_name: Text,
    permissions: Vec<Text>,
}
#[derive(Clone, Debug, serde::Deserialize)]
struct SignInRes {
    user: AuthenticatedAdmin,
}
#[derive(serde::Serialize)]
struct SignInReq {
    login: Text,
    password: Text,
}
#[derive(Clone, Debug, serde::Deserialize)]
struct User {
    display_name: Text,
    id: Id,
    is_banned: bool,
    login: Text,
}
#[derive(Clone, Debug, serde::Deserialize)]
struct Role {
    id: Id,
    is_system: bool,
    name: Text,
}
#[derive(Clone, Debug, serde::Deserialize)]
struct Permission {
    id: Id,
    name: Text,
}
#[derive(Clone, Debug, serde::Deserialize)]
struct AuditEntry {
    action: Text,
    created_at: Text,
    resource: Text,
    succeeded: bool,
    user_id: Option<Id>,
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Settings {
    default_admin_route: Text,
    main_logo: Option<Text>,
    organization_contacts: Option<Text>,
    organization_name: Option<Text>,
    primary_color: Option<Text>,
    site_name: Text,
    support_url: Option<Text>,
    tab_title: Option<Text>,
}
#[derive(Clone, Debug, serde::Deserialize)]
struct GitInfo {
    commit: Option<Text>,
}
#[derive(Clone, Debug)]
enum Page {
    Loading,
    Users(Vec<User>),
    Roles(Vec<Role>),
    Permissions(Vec<Permission>),
    Audit(Vec<AuditEntry>),
    Settings(Settings),
    Text(Text),
    Error(Text),
}
#[derive(Clone, Debug, thiserror::Error)]
enum ApiEr {
    #[error("request failed: {0}")]
    Request(Text),
    #[error("server returned HTTP {0}")]
    Status(u16),
}
#[derive(Clone, Debug)]
struct AdminApiClient {}
impl AdminApiClient {
    const fn new() -> Self {
        Self {}
    }
    async fn get<Output>(&self, path: &str) -> Result<Output, ApiEr>
    where
        Output: serde::de::DeserializeOwned,
    {
        self.decode(gloo_net::http::Request::get(path).send().await)
            .await
    }
    async fn send_json<Input>(
        self,
        method: gloo_net::http::Method,
        path: String,
        input: Input,
    ) -> Result<(), ApiEr>
    where
        Input: serde::Serialize,
    {
        let mut request = gloo_net::http::RequestBuilder::new(&path).method(method);
        if let Some(token) = csrf_token() {
            request = request.header("X-CSRF-Token", &token.0);
        }
        let request = request
            .json(&input)
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        self.empty(request.send().await).await
    }
    async fn send(self, method: gloo_net::http::Method, path: String) -> Result<(), ApiEr> {
        let mut request = gloo_net::http::RequestBuilder::new(&path).method(method);
        if let Some(token) = csrf_token() {
            request = request.header("X-CSRF-Token", &token.0);
        }
        self.empty(request.send().await).await
    }
    async fn decode<Output>(
        &self,
        response: Result<gloo_net::http::Response, gloo_net::Error>,
    ) -> Result<Output, ApiEr>
    where
        Output: serde::de::DeserializeOwned,
    {
        let response = response.map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        if !(200..300).contains(&response.status()) {
            return Err(ApiEr::Status(response.status()));
        }
        response
            .json()
            .await
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))
    }
    async fn empty(
        &self,
        response: Result<gloo_net::http::Response, gloo_net::Error>,
    ) -> Result<(), ApiEr> {
        let response = response.map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        if (200..300).contains(&response.status()) {
            Ok(())
        } else {
            Err(ApiEr::Status(response.status()))
        }
    }
    async fn audit(&self) -> Result<Vec<AuditEntry>, ApiEr> {
        self.get("/api/v1/admin/audit-log").await
    }
    async fn me(&self) -> Result<AuthenticatedAdmin, ApiEr> {
        self.get("/api/v1/admin/auth/me").await
    }
    async fn permissions(&self) -> Result<Vec<Permission>, ApiEr> {
        self.get("/api/v1/admin/permissions").await
    }
    async fn roles(&self) -> Result<Vec<Role>, ApiEr> {
        self.get("/api/v1/admin/roles").await
    }
    async fn settings(&self) -> Result<Settings, ApiEr> {
        self.get("/api/v1/admin/system-settings").await
    }
    async fn users(&self) -> Result<Vec<User>, ApiEr> {
        self.get("/api/v1/admin/users").await
    }
    async fn metrics(&self) -> Result<Text, ApiEr> {
        let response = gloo_net::http::Request::get(concat!("/api/v1/admin", "/metrics"))
            .send()
            .await
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        if !(200..300).contains(&response.status()) {
            return Err(ApiEr::Status(response.status()));
        }
        response
            .text()
            .await
            .map(Text::from)
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))
    }
    async fn version(&self) -> Result<GitInfo, ApiEr> {
        self.get("/api/v1/git_info").await
    }
    async fn sign_in(&self, input: &SignInReq) -> Result<SignInRes, ApiEr> {
        let request = gloo_net::http::Request::post(concat!("/api/v1/admin", "/auth/sign-in"))
            .json(input)
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        self.decode(request.send().await).await
    }
}
fn browser_window() -> Option<web_sys::Window> {
    web_sys::window()
}
fn path() -> String {
    browser_window()
        .and_then(|value| value.location().pathname().ok())
        .unwrap_or_else(|| "/admin".to_owned())
}
fn redirect(path: &str) {
    if let Some(value) = browser_window() {
        let _result = value.location().set_href(path);
    }
}
fn prompt(label: &str, current: &str) -> Option<Text> {
    browser_window()
        .and_then(|value| {
            value
                .prompt_with_message_and_default(label, current)
                .ok()
                .flatten()
        })
        .map(Text::from)
}
fn ids(value: &str) -> Vec<i64> {
    value
        .split(',')
        .filter_map(|item| Id::try_from(item.trim()).ok())
        .map(|item| item.0)
        .collect()
}
fn csrf_token() -> Option<Text> {
    let document = browser_window()?
        .document()?
        .dyn_into::<web_sys::HtmlDocument>()
        .ok()?;
    let cookies = document.cookie().ok()?;
    cookies.split(';').map(str::trim).find_map(|cookie| {
        cookie
            .strip_prefix("admin_csrf_token=")
            .map(|value| Text::from(value.to_owned()))
    })
}
fn has_permission(auth: &Option<AuthenticatedAdmin>, permission: &str) -> bool {
    auth.as_ref()
        .is_some_and(|value| value.permissions.iter().any(|item| item.0 == permission))
}
fn load(client: AdminApiClient, page: RwSignal<Page>) {
    page.set(Page::Loading);
    leptos::task::spawn_local(async move {
        let result = match path().as_str() {
            "/admin/users" => client.users().await.map(Page::Users),
            "/admin/roles" => client.roles().await.map(Page::Roles),
            "/admin/permissions" => client.permissions().await.map(Page::Permissions),
            "/admin/audit-log" => client.audit().await.map(Page::Audit),
            "/admin/system-settings" => client.settings().await.map(Page::Settings),
            "/admin/metrics" => client.metrics().await.map(Page::Text),
            "/admin/version" => client.version().await.map(|value| {
                Page::Text(
                    value
                        .commit
                        .unwrap_or_else(|| Text::from("Unknown version".to_owned())),
                )
            }),
            _ => {
                redirect("/admin/version");
                return;
            }
        };
        page.set(result.unwrap_or_else(|error| Page::Error(Text::from(error.to_string()))));
    });
}
fn run_action<FutureValue>(future: FutureValue, client: AdminApiClient, page: RwSignal<Page>)
where
    FutureValue: Future<Output = Result<(), ApiEr>> + 'static,
{
    leptos::task::spawn_local(async move {
        match future.await {
            Ok(()) => load(client, page),
            Err(error) => page.set(Page::Error(Text::from(error.to_string()))),
        }
    });
}
#[component]
fn SignIn(client: AdminApiClient) -> impl IntoView {
    let login = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<Text>::None);
    view! {
        <main><h1>"Sign in"</h1>
        <form on:submit=move |event| {
            event.prevent_default();
            let client = client.clone();
            leptos::task::spawn_local(async move {
                let input = SignInReq { login: Text::from(login.get()), password: Text::from(password.get()) };
                match client.sign_in(&input).await {
                    Ok(value) => { let _display_name = value.user.display_name; redirect("/admin"); },
                    Err(value) => error.set(Some(Text::from(value.to_string()))),
                }
            });
        }>
        <input placeholder="Login" autocomplete="username" prop:value=move || login.get() on:input=move |event| login.set(event_target_value(&event)) />
        <input type="password" placeholder="Password" autocomplete="current-password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) />
        <button type="submit">"Sign in"</button>
        </form>
        {move || error.get().map(|value| view! { <p class="error">{value.to_string()}</p> })}
        </main>
    }
}
#[component]
fn Shell(auth: AuthenticatedAdmin, client: AdminApiClient) -> impl IntoView {
    let auth = Some(auth);
    let page = RwSignal::new(Page::Loading);
    load(client.clone(), page);
    let nav = [
        ("/admin/users", "Users", "users:read"),
        ("/admin/roles", "Roles", "roles:read"),
        ("/admin/permissions", "Permissions", "permissions:read"),
        ("/admin/audit-log", "Audit log", "audit_log:read"),
        ("/admin/system-settings", "Settings", "system_settings:read"),
        ("/admin/metrics", "Metrics", "metrics:read"),
        ("/admin/version", "Version", ""),
        ("/admin/swagger-ui", "API", "openapi:read"),
    ];
    let client_for_sign_out = client.clone();
    view! {
        <header><strong>"Admin"</strong><nav>
        {nav.into_iter().filter(|item| item.2.is_empty() || has_permission(&auth, item.2)).map(|item| view! { <a href=item.0>{item.1}</a> }).collect_view()}
        </nav><span>{auth.as_ref().map(|value| value.display_name.to_string())}
        <a href="#" on:click=move |event| { event.prevent_default(); let client = client_for_sign_out.clone(); leptos::task::spawn_local(async move { if client.send(gloo_net::http::Method::POST, concat!("/api/v1/admin", "/auth/sign-out").to_owned()).await.is_ok() { redirect("/admin/sign-in"); } }); }>"Sign out"</a></span></header>
        <main><PageView page client auth /></main>
    }
}
#[component]
fn PageView(
    page: RwSignal<Page>,
    client: AdminApiClient,
    auth: Option<AuthenticatedAdmin>,
) -> impl IntoView {
    move || {
        match page.get() {
        Page::Loading => view! { <p>"Loading..."</p> }.into_any(),
        Page::Error(value) => view! { <p class="error">{value.to_string()}</p> }.into_any(),
        Page::Text(value) => view! { <pre>{value.to_string()}</pre> }.into_any(),
        Page::Users(values) => users_view(values, client.clone(), page, &auth).into_any(),
        Page::Roles(values) => roles_view(values, client.clone(), page, &auth).into_any(),
        Page::Permissions(values) => view! { <section><h1>"Permissions"</h1><table><thead><tr><th>"ID"</th><th>"Name"</th></tr></thead><tbody>{values.into_iter().map(|value| view! { <tr><td>{value.id.to_string()}</td><td>{value.name.to_string()}</td></tr> }).collect_view()}</tbody></table></section> }.into_any(),
        Page::Audit(values) => view! { <section><h1>"Audit log"</h1><table><thead><tr><th>"Time"</th><th>"User"</th><th>"Action"</th><th>"Resource"</th><th>"Result"</th></tr></thead><tbody>{values.into_iter().map(|value| view! { <tr><td>{value.created_at.to_string()}</td><td>{value.user_id.map(|id| id.to_string()).unwrap_or_default()}</td><td>{value.action.to_string()}</td><td>{value.resource.to_string()}</td><td>{value.succeeded.to_string()}</td></tr> }).collect_view()}</tbody></table></section> }.into_any(),
        Page::Settings(value) => settings_view(value, client.clone(), page, &auth).into_any(),
    }
    }
}
fn users_view(
    values: Vec<User>,
    client: AdminApiClient,
    page: RwSignal<Page>,
    auth: &Option<AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create = has_permission(auth, "users:create");
    let client_for_create = client.clone();
    view! { <section><h1>"Users"</h1>
    <button disabled=!can_create on:click=move |_| { if let (Some(login), Some(display_name), Some(password)) = (prompt("Login", ""), prompt("Display name", ""), prompt("Password", "")) { let body = serde_json::json!({"login": login.0, "display_name": display_name.0, "password": password.0}); let action_client = client_for_create.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::POST, concat!("/api/v1/admin", "/users").to_owned(), body), action_client, page); } }>"Create user"</button>
    <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Actions"</th></tr></thead><tbody>
    {values.into_iter().map(|value| { let edit_client = client.clone(); let ban_client = client.clone(); let password_client = client.clone(); let roles_client = client.clone(); let delete_client = client.clone(); let id = value.id; let edit_login = value.login.clone(); let edit_display_name = value.display_name.clone(); let delete_login = value.login.clone(); let is_banned = value.is_banned; view! { <tr><td>{id.to_string()}</td><td>{value.login.to_string()}</td><td>{value.display_name.to_string()}</td><td>{is_banned.to_string()}</td><td>
    <button disabled=!has_permission(auth, "users:update") on:click=move |_| { if let (Some(login), Some(display_name)) = (prompt("Login", &edit_login.0), prompt("Display name", &edit_display_name.0)) { let body = serde_json::json!({"login": login.0, "display_name": display_name.0}); let action_client = edit_client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::PATCH, format!("{API}/users/{id}"), body), action_client, page); } }>"Edit"</button>
    <button disabled=!has_permission(auth, "users:update") on:click=move |_| { let body = serde_json::json!({"is_banned": !is_banned}); let action_client = ban_client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::POST, format!("{API}/users/{id}/ban"), body), action_client, page); }>{if is_banned { "Unban" } else { "Ban" }}</button>
    <button disabled=!has_permission(auth, "users:update") on:click=move |_| { if let Some(password) = prompt("New password", "") { let body = serde_json::json!({"password": password.0}); let action_client = password_client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::POST, format!("{API}/users/{id}/password"), body), action_client, page); } }>"Password"</button>
    <button disabled=!has_permission(auth, "user_roles:update") on:click=move |_| { if let Some(value) = prompt("Role IDs separated by commas", "") { let body = serde_json::json!({"role_ids": ids(&value.0)}); let action_client = roles_client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::PUT, format!("{API}/users/{id}/roles"), body), action_client, page); } }>"Roles"</button>
    <button disabled=!has_permission(auth, "users:delete") on:click=move |_| { let confirmed = browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_login}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); run_action(action_client.clone().send(gloo_net::http::Method::DELETE, format!("{API}/users/{id}")), action_client, page); } }>"Delete"</button>
    </td></tr> } }).collect_view()}
    </tbody></table></section> }
}
fn roles_view(
    values: Vec<Role>,
    client: AdminApiClient,
    page: RwSignal<Page>,
    auth: &Option<AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create = has_permission(auth, "roles:create");
    let client_for_create = client.clone();
    view! { <section><h1>"Roles"</h1><button disabled=!can_create on:click=move |_| { if let Some(name) = prompt("Name", "") { let body = serde_json::json!({"name": name.0}); let action_client = client_for_create.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::POST, concat!("/api/v1/admin", "/roles").to_owned(), body), action_client, page); } }>"Create role"</button>
    <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Actions"</th></tr></thead><tbody>{values.into_iter().map(|value| { let edit_client = client.clone(); let permissions_client = client.clone(); let delete_client = client.clone(); let id = value.id; let edit_name = value.name.clone(); let delete_name = value.name.clone(); view! { <tr><td>{id.to_string()}</td><td>{value.name.to_string()}</td><td>{value.is_system.to_string()}</td><td><button disabled=!has_permission(auth, "roles:update") on:click=move |_| { if let Some(name) = prompt("Name", &edit_name.0) { let body = serde_json::json!({"name": name.0}); let action_client = edit_client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::PATCH, format!("{API}/roles/{id}"), body), action_client, page); } }>"Edit"</button><button disabled=!has_permission(auth, "role_permissions:update") on:click=move |_| { if let Some(value) = prompt("Permission IDs separated by commas", "") { let body = serde_json::json!({"permission_ids": ids(&value.0)}); let action_client = permissions_client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::PUT, format!("{API}/roles/{id}/permissions"), body), action_client, page); } }>"Permissions"</button><button disabled=!has_permission(auth, "roles:delete") on:click=move |_| { let confirmed = browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_name}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); run_action(action_client.clone().send(gloo_net::http::Method::DELETE, format!("{API}/roles/{id}")), action_client, page); } }>"Delete"</button></td></tr> } }).collect_view()}</tbody></table></section> }
}
fn settings_view(
    value: Settings,
    client: AdminApiClient,
    page: RwSignal<Page>,
    auth: &Option<AuthenticatedAdmin>,
) -> impl IntoView {
    let site_name = RwSignal::new(value.site_name.0);
    let default_admin_route = RwSignal::new(value.default_admin_route.0);
    let can_update = has_permission(auth, "system_settings:update");
    view! { <section><h1>"Settings"</h1><form on:submit=move |event| { event.prevent_default(); let body = serde_json::json!({"site_name": site_name.get(), "default_admin_route": default_admin_route.get()}); let action_client = client.clone(); run_action(action_client.clone().send_json(gloo_net::http::Method::PATCH, concat!("/api/v1/admin", "/system-settings").to_owned(), body), action_client, page); }><input placeholder="site_name" prop:value=move || site_name.get() on:input=move |event| site_name.set(event_target_value(&event)) /><input placeholder="default_admin_route" prop:value=move || default_admin_route.get() on:input=move |event| default_admin_route.set(event_target_value(&event)) /><button type="submit" disabled=!can_update>"Save"</button></form></section> }
}
#[component]
pub fn App() -> impl IntoView {
    let client = AdminApiClient::new();
    let auth = LocalResource::new({
        let client = client.clone();
        move || {
            let client = client.clone();
            async move { client.me().await }
        }
    });
    if path() == "/admin/sign-in" {
        return view! { <SignIn client /> }.into_any();
    }
    let client_for_auth = client.clone();
    view! { <Suspense fallback=move || view! { <main><p>"Loading..."</p></main> }>{move || { let client = client_for_auth.clone(); Suspend::new(async move { match auth.await { Ok(value) => view! { <Shell auth=value client=client.clone() /> }.into_any(), Err(_) => { redirect("/admin/sign-in"); view! { <main></main> }.into_any() } } }) }}</Suspense> }.into_any()
}
