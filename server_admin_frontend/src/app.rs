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
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, newtype::Newtype)]
#[serde(transparent)]
#[newtype(display)]
struct Text(String);
impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, serde::Deserialize)]
struct GitInfo {
    commit: Option<Text>,
}
#[derive(Clone, Debug)]
enum Page {
    Loading,
    Users(Vec<server_admin_contract::AdminUserSummary>),
    Roles(Vec<server_admin_contract::AdminRoleSummary>),
    Permissions(Vec<server_admin_contract::AdminPermissionSummary>),
    Audit(Vec<server_admin_contract::AdminAuditView>),
    Settings(server_admin_contract::AdminSettingsView),
    Text(Text),
    Error(Text),
}
#[derive(Clone, Debug, thiserror::Error)]
enum ApiEr {
    #[error("request failed: {0}")]
    Request(Text),
    #[error("server returned HTTP {0}: {1}")]
    Status(u16, Text),
}
#[derive(Clone)]
struct AdminApiClient {
    auth_refresh: std::sync::Arc<std::sync::RwLock<AuthRefreshCoordinator>>,
    transport: crate::transport::GlooTransport,
}
#[derive(Default)]
struct AuthRefreshCoordinator {
    state: crate::auth_keep_alive::AuthRefreshState,
    waiters: Vec<futures::channel::oneshot::Sender<Result<(), ApiEr>>>,
}
enum AuthRefreshWork {
    Start,
    Join(futures::channel::oneshot::Receiver<Result<(), ApiEr>>),
}
fn auth_refresh_state_er() -> ApiEr {
    ApiEr::Request(Text::from(
        "authentication refresh state is unavailable".to_owned(),
    ))
}
impl AdminApiClient {
    fn new() -> Self {
        Self {
            auth_refresh: std::sync::Arc::from(std::sync::RwLock::new(
                AuthRefreshCoordinator::default(),
            )),
            transport: crate::transport::GlooTransport,
        }
    }
    async fn get<Output>(&self, route: server_admin_contract::AdminRoute) -> Result<Output, ApiEr>
    where
        Output: serde::de::DeserializeOwned,
    {
        let response = self.transport_response(route, Vec::new()).await?;
        serde_json::from_slice(response.body().as_ref())
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))
    }
    async fn send_json<Input>(
        self,
        route: server_admin_contract::AdminRoute,
        input: Input,
    ) -> Result<(), ApiEr>
    where
        Input: serde::Serialize,
    {
        let body = serde_json::to_vec(&input)
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        let _response = self.transport_response(route, body).await?;
        Ok(())
    }
    async fn send(self, route: server_admin_contract::AdminRoute) -> Result<(), ApiEr> {
        let _response = self.transport_response(route, Vec::new()).await?;
        Ok(())
    }
    async fn transport_response(
        &self,
        route: server_admin_contract::AdminRoute,
        body: Vec<u8>,
    ) -> Result<frontend_contract::TransportResponse, ApiEr> {
        let response = self.transport_response_once(route, body.as_slice()).await?;
        if u16::from(response.status()) == 401u16
            && !matches!(
                route,
                server_admin_contract::AdminRoute::Refresh
                    | server_admin_contract::AdminRoute::SignIn
            )
        {
            self.refresh_session().await?;
            return self
                .transport_response_once(route, body.as_slice())
                .await
                .and_then(|retried| {
                    let expected = route.contract().success_status().transport_status();
                    if retried.status() == expected {
                        Ok(retried)
                    } else {
                        Err(response_er(retried.status(), retried.body()))
                    }
                });
        }
        let expected = route.contract().success_status().transport_status();
        if response.status() != expected {
            return Err(response_er(response.status(), response.body()));
        }
        Ok(response)
    }
    async fn transport_response_once(
        &self,
        route: server_admin_contract::AdminRoute,
        body: &[u8],
    ) -> Result<frontend_contract::TransportResponse, ApiEr> {
        Self::send_once(self.transport, route, body).await
    }
    async fn send_once(
        transport: crate::transport::GlooTransport,
        route: server_admin_contract::AdminRoute,
        body: &[u8],
    ) -> Result<frontend_contract::TransportResponse, ApiEr> {
        let path = route.path();
        let request = frontend_contract::TransportRequest::new(
            frontend_contract::TransportBody::from(body.to_vec()),
            frontend_contract::TransportPath::try_from(path.as_ref().to_owned())
                .map_err(|error| ApiEr::Request(Text::from(error.to_string())))?,
            route.contract(),
        );
        frontend_contract::Transport::send(&transport, request)
            .await
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))
    }
    async fn refresh_session(&self) -> Result<(), ApiEr> {
        let now = crate::auth_keep_alive::StdAuthRefreshInstant::now();
        let work = match self
            .auth_refresh
            .write()
            .map_err(|_error| auth_refresh_state_er())?
            .state
            .begin(now)
        {
            crate::auth_keep_alive::AuthRefreshBegin::Start => AuthRefreshWork::Start,
            crate::auth_keep_alive::AuthRefreshBegin::Join => {
                let (sender, receiver) = futures::channel::oneshot::channel();
                self.auth_refresh
                    .write()
                    .map_err(|_error| auth_refresh_state_er())?
                    .waiters
                    .push(sender);
                AuthRefreshWork::Join(receiver)
            }
            crate::auth_keep_alive::AuthRefreshBegin::Rejected => {
                redirect("/admin/sign-in");
                return Err(ApiEr::Status(
                    401u16,
                    Text::from("authentication refresh rejected".to_owned()),
                ));
            }
            crate::auth_keep_alive::AuthRefreshBegin::Wait => {
                return Err(ApiEr::Request(Text::from(
                    "authentication refresh retry is delayed".to_owned(),
                )));
            }
        };
        if let AuthRefreshWork::Join(receiver) = work {
            return receiver.await.map_err(|_error| auth_refresh_state_er())?;
        }
        let response = Self::send_once(
            self.transport,
            server_admin_contract::AdminRoute::Refresh,
            &[],
        )
        .await;
        let result = response.and_then(|value| {
            let expected = server_admin_contract::AdminRoute::Refresh
                .contract()
                .success_status()
                .transport_status();
            if value.status() == expected {
                Ok(())
            } else {
                Err(response_er(value.status(), value.body()))
            }
        });
        let outcome = match &result {
            Ok(()) => crate::auth_keep_alive::AuthRefreshOutcome::Refreshed,
            Err(ApiEr::Status(401u16 | 403u16, _detail)) => {
                crate::auth_keep_alive::AuthRefreshOutcome::Rejected
            }
            Err(ApiEr::Request(_) | ApiEr::Status(_, _)) => {
                crate::auth_keep_alive::AuthRefreshOutcome::TemporaryFailure
            }
        };
        let waiters = {
            let mut coordinator = self
                .auth_refresh
                .write()
                .map_err(|_error| auth_refresh_state_er())?;
            coordinator.state.finish(outcome, now);
            std::mem::take(&mut coordinator.waiters)
        };
        waiters.into_iter().for_each(|sender| {
            let _send_result = sender.send(result.clone());
        });
        if matches!(
            outcome,
            crate::auth_keep_alive::AuthRefreshOutcome::Rejected
        ) {
            redirect("/admin/sign-in");
        }
        result
    }
    async fn audit(&self) -> Result<Vec<server_admin_contract::AdminAuditView>, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Audit).await
    }
    async fn me(&self) -> Result<server_admin_contract::AuthenticatedAdmin, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Me).await
    }
    async fn permissions(
        &self,
    ) -> Result<Vec<server_admin_contract::AdminPermissionSummary>, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Permissions)
            .await
    }
    async fn roles(&self) -> Result<Vec<server_admin_contract::AdminRoleSummary>, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Roles).await
    }
    async fn settings(&self) -> Result<server_admin_contract::AdminSettingsView, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Settings).await
    }
    async fn users(&self) -> Result<Vec<server_admin_contract::AdminUserSummary>, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Users).await
    }
    async fn metrics(&self) -> Result<Text, ApiEr> {
        let route = server_admin_contract::AdminRoute::Metrics;
        let response = self.transport_response(route, Vec::new()).await?;
        String::from_utf8(response.body().as_ref().to_vec())
            .map(Text::from)
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))
    }
    async fn version(&self) -> Result<GitInfo, ApiEr> {
        self.get(server_admin_contract::AdminRoute::Version).await
    }
    async fn sign_in(
        &self,
        input: server_admin_contract::AdminSignInReq,
    ) -> Result<server_admin_contract::AdminSignInRes, ApiEr> {
        let route = server_admin_contract::AdminRoute::SignIn;
        let body = serde_json::to_vec(&input)
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))?;
        let response = self.transport_response(route, body).await?;
        serde_json::from_slice(response.body().as_ref())
            .map_err(|error| ApiEr::Request(Text::from(error.to_string())))
    }
}
fn response_er(
    status: frontend_contract::TransportStatus,
    body: &frontend_contract::TransportBody,
) -> ApiEr {
    let detail = frontend_contract::decode_api_problem(body).map_or_else(
        || Text::from("request failed".to_owned()),
        |problem| Text::from(problem.detail().as_ref().to_owned()),
    );
    ApiEr::Status(u16::from(status), detail)
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
fn role_ids(value: &str) -> Vec<server_admin_contract::AdminRoleId> {
    value
        .split(',')
        .filter_map(|item| item.trim().parse::<i64>().ok())
        .map(server_admin_contract::AdminRoleId::from)
        .collect()
}
fn permission_ids(value: &str) -> Vec<server_admin_contract::AdminPermissionId> {
    value
        .split(',')
        .filter_map(|item| item.trim().parse::<i64>().ok())
        .map(server_admin_contract::AdminPermissionId::from)
        .collect()
}
fn has_permission(
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
    permission: &str,
) -> bool {
    auth.as_ref().is_some_and(|value| {
        value
            .permissions()
            .iter()
            .any(|item| item.as_ref() == permission)
    })
}
fn has_route_permission(
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
    route: server_admin_contract::AdminRoute,
) -> bool {
    match route.contract().authentication() {
        frontend_contract::AuthenticationRequirement::Authenticated => auth.is_some(),
        frontend_contract::AuthenticationRequirement::Permission(permission) => {
            has_permission(auth, permission.as_ref())
        }
        frontend_contract::AuthenticationRequirement::Public => true,
    }
}
fn has_page_permission(
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
    page: server_admin_contract::AdminPage,
) -> bool {
    match page.authentication() {
        frontend_contract::AuthenticationRequirement::Authenticated => auth.is_some(),
        frontend_contract::AuthenticationRequirement::Permission(permission) => {
            has_permission(auth, permission.as_ref())
        }
        frontend_contract::AuthenticationRequirement::Public => true,
    }
}
fn load(client: AdminApiClient, page: RwSignal<Page>) {
    page.set(Page::Loading);
    leptos::task::spawn_local(async move {
        let current_path = path();
        let current_page = server_admin_contract::AdminPage::from_path(
            server_admin_contract::AdminPagePathRef::from(current_path.as_str()),
        );
        let result = match current_page {
            Some(server_admin_contract::AdminPage::Users) => client.users().await.map(Page::Users),
            Some(server_admin_contract::AdminPage::Roles) => client.roles().await.map(Page::Roles),
            Some(server_admin_contract::AdminPage::Permissions) => {
                client.permissions().await.map(Page::Permissions)
            }
            Some(server_admin_contract::AdminPage::Audit) => client.audit().await.map(Page::Audit),
            Some(server_admin_contract::AdminPage::Settings) => {
                client.settings().await.map(Page::Settings)
            }
            Some(server_admin_contract::AdminPage::Metrics) => {
                client.metrics().await.map(Page::Text)
            }
            Some(server_admin_contract::AdminPage::Version) => {
                client.version().await.map(|value| {
                    Page::Text(
                        value
                            .commit
                            .unwrap_or_else(|| Text::from("Unknown version".to_owned())),
                    )
                })
            }
            Some(server_admin_contract::AdminPage::OpenApi) | None => {
                redirect(server_admin_contract::AdminPage::Version.path().as_ref());
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
    let pending = RwSignal::new(false);
    view! {
        <main class="auth-page"><section class="auth-card"><div class="auth-brand"><span class="brand-mark">"A"</span><div><strong>"Admin Console"</strong><small>"Workspace control center"</small></div></div><div class="auth-copy"><p class="eyebrow">"Secure access"</p><h1>"Welcome back"</h1><p>"Sign in to manage users, roles and system settings."</p></div>
        <form class="auth-form" on:submit=move |event| {
            event.prevent_default();
            let client = client.clone();
            pending.set(true);
            error.set(None);
            leptos::task::spawn_local(async move {
                let input = match (server_admin_contract::AdminLogin::try_from(login.get()), server_admin_contract::AdminPassword::try_from(password.get())) {
                    (Ok(login), Ok(password)) => server_admin_contract::AdminSignInReq::new(login, password),
                    (Err(value), _) => { pending.set(false); error.set(Some(Text::from(value.to_string()))); return; },
                    (_, Err(value)) => { pending.set(false); error.set(Some(Text::from(value.to_string()))); return; },
                };
                match client.sign_in(input).await {
                    Ok(value) => { let _display_name = value.user().display_name(); redirect("/admin"); },
                    Err(value) => { pending.set(false); error.set(Some(Text::from(value.to_string()))); },
                }
            });
        }>
        <label><span>"Login"</span><input placeholder="Enter your login" autocomplete="username" prop:value=move || login.get() on:input=move |event| login.set(event_target_value(&event)) /></label>
        <label><span>"Password"</span><input type="password" placeholder="Enter your password" autocomplete="current-password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) /></label>
        <button class="primary-button" type="submit" disabled=move || pending.get()>{move || if pending.get() { "Signing in..." } else { "Sign in" }}</button>
        </form>
        {move || error.get().map(|value| view! { <div class="alert error" role="alert"><strong>"Unable to sign in"</strong><span>{value.to_string()}</span></div> })}
        <p class="auth-footnote">"Protected administrator area"</p></section><div class="auth-visual"><div class="visual-orb"></div><div class="visual-card"><span>"SYSTEM STATUS"</span><strong>"All controls in one place"</strong><p>"Fast, typed and secure administration powered by Rust."</p></div></div></main>
    }
}
#[component]
fn Shell(auth: server_admin_contract::AuthenticatedAdmin, client: AdminApiClient) -> impl IntoView {
    let auth = Some(auth);
    let page = RwSignal::new(Page::Loading);
    let nav = server_admin_contract::AdminPage::ALL;
    if path() == "/admin" {
        let target = nav
            .iter()
            .find(|item| has_page_permission(&auth, **item))
            .copied()
            .unwrap_or(server_admin_contract::AdminPage::Version);
        redirect(target.path().as_ref());
    } else {
        load(client.clone(), page);
    }
    let client_for_sign_out = client.clone();
    let current_path = path();
    view! {
        <div class="app-shell"><header class="sidebar"><a class="brand" href="/admin"><span class="brand-mark">"A"</span><span><strong>"Admin"</strong><small>"Control center"</small></span></a><nav>
        {nav.into_iter().filter(|item| has_page_permission(&auth, *item)).map(|item| { let item_path = item.path().as_ref().to_owned(); let item_title = item.title().as_ref().to_owned(); let active = current_path == item_path; view! { <a class:active=active href=item_path><span class="nav-dot"></span>{item_title}</a> } }).collect_view()}
        </nav><div class="profile"><div class="avatar">{auth.as_ref().and_then(|value| value.display_name().as_ref().chars().next()).unwrap_or('A').to_string()}</div><div><strong>{auth.as_ref().map(|value| value.display_name().to_string())}</strong><small>"Administrator"</small></div><button class="icon-button" title="Sign out" on:click=move |event| { event.prevent_default(); let client = client_for_sign_out.clone(); leptos::task::spawn_local(async move { if client.send(server_admin_contract::AdminRoute::SignOut).await.is_ok() { redirect("/admin/sign-in"); } }); }>"Exit"</button></div></header>
        <main class="content"><PageView page client auth /></main></div>
    }
}
#[component]
fn PageView(
    page: RwSignal<Page>,
    client: AdminApiClient,
    auth: Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    move || {
        match page.get() {
        Page::Loading => view! { <div class="loading-state"><span class="spinner"></span><strong>"Loading workspace"</strong><p>"Fetching the latest data..."</p></div> }.into_any(),
        Page::Error(value) => view! { <div class="alert error page-alert" role="alert"><strong>"Something went wrong"</strong><span>{value.to_string()}</span></div> }.into_any(),
        Page::Text(value) => view! { <section><div class="page-heading"><div><p class="eyebrow">"System"</p><h1>"Runtime information"</h1></div></div><div class="code-card"><pre>{value.to_string()}</pre></div></section> }.into_any(),
        Page::Users(values) => users_view(values, client.clone(), page, &auth).into_any(),
        Page::Roles(values) => roles_view(values, client.clone(), page, &auth).into_any(),
        Page::Permissions(values) => view! { <section><h1>"Permissions"</h1><table><thead><tr><th>"ID"</th><th>"Name"</th></tr></thead><tbody>{values.into_iter().map(|value| view! { <tr><td>{value.id().to_string()}</td><td>{value.name().to_string()}</td></tr> }).collect_view()}</tbody></table></section> }.into_any(),
        Page::Audit(values) => view! { <section><h1>"Audit log"</h1><table><thead><tr><th>"Time"</th><th>"User"</th><th>"Action"</th><th>"Resource"</th><th>"Result"</th></tr></thead><tbody>{values.into_iter().map(|value| view! { <tr><td>{value.created_at().to_string()}</td><td>{value.user_id().map(|id| id.to_string()).unwrap_or_default()}</td><td>{value.action().to_string()}</td><td>{value.resource().to_string()}</td><td>{value.succeeded().to_string()}</td></tr> }).collect_view()}</tbody></table></section> }.into_any(),
        Page::Settings(value) => settings_view(value, client.clone(), page, &auth).into_any(),
    }
    }
}
fn users_view(
    values: Vec<server_admin_contract::AdminUserSummary>,
    client: AdminApiClient,
    page: RwSignal<Page>,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create = has_route_permission(auth, server_admin_contract::AdminRoute::CreateUser);
    let client_for_create = client.clone();
    let content = view! { <div class="crud-content">
    <button disabled=!can_create on:click=move |_| { if let (Some(login), Some(display_name), Some(password)) = (prompt("Login", ""), prompt("Display name", ""), prompt("Password", "")) && let (Ok(login), Ok(display_name), Ok(password)) = (server_admin_contract::AdminLogin::try_from(login.0), server_admin_contract::AdminDisplayName::try_from(display_name.0), server_admin_contract::AdminPassword::try_from(password.0)) { let body = server_admin_contract::AdminCreateUserReq::new(display_name, login, password); let action_client = client_for_create.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateUser, body), action_client, page); } }>"Create user"</button>
    <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Actions"</th></tr></thead><tbody>
    {values.into_iter().map(|value| { let edit_client = client.clone(); let ban_client = client.clone(); let password_client = client.clone(); let roles_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_login = value.login().clone(); let edit_display_name = value.display_name().clone(); let delete_login = value.login().clone(); let is_banned = bool::from(value.is_banned()); view! { <tr><td>{id.to_string()}</td><td>{value.login().to_string()}</td><td>{value.display_name().to_string()}</td><td>{is_banned.to_string()}</td><td>
    <button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::UpdateUser(id)) on:click=move |_| { if let (Some(login), Some(display_name)) = (prompt("Login", edit_login.as_ref()), prompt("Display name", edit_display_name.as_ref())) && let (Ok(login), Ok(display_name)) = (server_admin_contract::AdminLogin::try_from(login.0), server_admin_contract::AdminDisplayName::try_from(display_name.0)) { let body = server_admin_contract::AdminUpdateUserReq::new(Some(display_name), Some(login)); let action_client = edit_client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateUser(id), body), action_client, page); } }>"Edit"</button>
    <button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::SetUserBan(id)) on:click=move |_| { let body = server_admin_contract::AdminSetUserBanReq::new(server_admin_contract::AdminBool::from(!is_banned)); let action_client = ban_client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserBan(id), body), action_client, page); }>{if is_banned { "Unban" } else { "Ban" }}</button>
    <button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::SetUserPassword(id)) on:click=move |_| { if let Some(password) = prompt("New password", "") && let Ok(password) = server_admin_contract::AdminPassword::try_from(password.0) { let body = server_admin_contract::AdminSetUserPasswordReq::new(password); let action_client = password_client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserPassword(id), body), action_client, page); } }>"Password"</button>
    <button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::SetUserRoles(id)) on:click=move |_| { if let Some(value) = prompt("Role IDs separated by commas", "") { let body = server_admin_contract::AdminSetUserRolesReq::from_ids(role_ids(&value.0)); let action_client = roles_client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserRoles(id), body), action_client, page); } }>"Roles"</button>
    <button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::DeleteUser(id)) on:click=move |_| { let confirmed = browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_login}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); run_action(action_client.clone().send(server_admin_contract::AdminRoute::DeleteUser(id)), action_client, page); } }>"Delete"</button>
    </td></tr> } }).collect_view()}
    </tbody></table></div> };
    crud_page(server_admin_contract::AdminPage::Users, content)
}
fn roles_view(
    values: Vec<server_admin_contract::AdminRoleSummary>,
    client: AdminApiClient,
    page: RwSignal<Page>,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create = has_route_permission(auth, server_admin_contract::AdminRoute::CreateRole);
    let client_for_create = client.clone();
    let content = view! { <section class="crud-content"><button disabled=!can_create on:click=move |_| { if let Some(name) = prompt("Name", "") && let Ok(name) = server_admin_contract::AdminRoleName::try_from(name.0) { let body = server_admin_contract::AdminCreateRoleReq::new(name); let action_client = client_for_create.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateRole, body), action_client, page); } }>"Create role"</button>
    <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Actions"</th></tr></thead><tbody>{values.into_iter().map(|value| { let edit_client = client.clone(); let permissions_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_name = value.name().clone(); let delete_name = value.name().clone(); view! { <tr><td>{id.to_string()}</td><td>{value.name().to_string()}</td><td>{value.is_system().to_string()}</td><td><button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::UpdateRole(id)) on:click=move |_| { if let Some(name) = prompt("Name", edit_name.as_ref()) && let Ok(name) = server_admin_contract::AdminRoleName::try_from(name.0) { let body = server_admin_contract::AdminUpdateRoleReq::new(name); let action_client = edit_client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateRole(id), body), action_client, page); } }>"Edit"</button><button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::SetRolePermissions(id)) on:click=move |_| { if let Some(value) = prompt("Permission IDs separated by commas", "") { let body = server_admin_contract::AdminSetRolePermissionsReq::from_ids(permission_ids(&value.0)); let action_client = permissions_client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetRolePermissions(id), body), action_client, page); } }>"Permissions"</button><button disabled=!has_route_permission(auth, server_admin_contract::AdminRoute::DeleteRole(id)) on:click=move |_| { let confirmed = browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_name}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); run_action(action_client.clone().send(server_admin_contract::AdminRoute::DeleteRole(id)), action_client, page); } }>"Delete"</button></td></tr> } }).collect_view()}</tbody></table></section> };
    crud_page(server_admin_contract::AdminPage::Roles, content)
}
fn crud_page(page: server_admin_contract::AdminPage, content: impl IntoView) -> impl IntoView {
    view! { <section><div class="page-heading"><div><p class="eyebrow">"Administration"</p><h1>{page.title().as_ref().to_owned()}</h1></div></div>{content}</section> }
}
fn settings_view(
    value: server_admin_contract::AdminSettingsView,
    client: AdminApiClient,
    page: RwSignal<Page>,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let site_name = RwSignal::new(value.site_name().as_ref().to_owned());
    let default_admin_route = RwSignal::new(value.default_admin_route().as_ref().to_owned());
    let tab_title = RwSignal::new(
        value
            .tab_title()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let main_logo = RwSignal::new(
        value
            .main_logo()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let primary_color = RwSignal::new(
        value
            .primary_color()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let organization_name = RwSignal::new(
        value
            .organization_name()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let organization_contacts = RwSignal::new(
        value
            .organization_contacts()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let support_url = RwSignal::new(
        value
            .support_url()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let can_update = has_route_permission(auth, server_admin_contract::AdminRoute::UpdateSettings);
    view! { <section><p class="eyebrow">"Configuration"</p><h1>"System settings"</h1><form class="settings-form" on:submit=move |event| { event.prevent_default(); if let (Ok(site_name), Ok(default_admin_route), Ok(tab_title), Ok(main_logo), Ok(primary_color), Ok(organization_name), Ok(organization_contacts), Ok(support_url)) = (server_admin_contract::AdminSettingText::try_from(site_name.get()), server_admin_contract::AdminSettingText::try_from(default_admin_route.get()), server_admin_contract::AdminSettingText::try_from(tab_title.get()), server_admin_contract::AdminSettingText::try_from(main_logo.get()), server_admin_contract::AdminSettingText::try_from(primary_color.get()), server_admin_contract::AdminSettingText::try_from(organization_name.get()), server_admin_contract::AdminSettingText::try_from(organization_contacts.get()), server_admin_contract::AdminSettingText::try_from(support_url.get())) { let body = server_admin_contract::AdminUpdateSettingsReq::new(Some(default_admin_route), Some(main_logo), Some(organization_contacts), Some(organization_name), Some(primary_color), Some(site_name), Some(support_url), Some(tab_title)); let action_client = client.clone(); run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateSettings, body), action_client, page); } }>
    <label><span>"Site name"</span><input placeholder="Administration" prop:value=move || site_name.get() on:input=move |event| site_name.set(event_target_value(&event)) /></label>
    <label><span>"Browser tab title"</span><input placeholder="Admin Console" prop:value=move || tab_title.get() on:input=move |event| tab_title.set(event_target_value(&event)) /></label>
    <label><span>"Default admin route"</span><input placeholder="/admin/users" prop:value=move || default_admin_route.get() on:input=move |event| default_admin_route.set(event_target_value(&event)) /></label>
    <label><span>"Primary color"</span><input placeholder="#6757e8" prop:value=move || primary_color.get() on:input=move |event| primary_color.set(event_target_value(&event)) /></label>
    <label class="full-field"><span>"Logo URL"</span><input placeholder="https://..." prop:value=move || main_logo.get() on:input=move |event| main_logo.set(event_target_value(&event)) /></label>
    <label><span>"Organization"</span><input placeholder="Organization name" prop:value=move || organization_name.get() on:input=move |event| organization_name.set(event_target_value(&event)) /></label>
    <label><span>"Contacts"</span><input placeholder="support@example.com" prop:value=move || organization_contacts.get() on:input=move |event| organization_contacts.set(event_target_value(&event)) /></label>
    <label class="full-field"><span>"Support URL"</span><input placeholder="https://support.example.com" prop:value=move || support_url.get() on:input=move |event| support_url.set(event_target_value(&event)) /></label>
    <button type="submit" disabled=!can_update>"Save changes"</button></form></section> }
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
