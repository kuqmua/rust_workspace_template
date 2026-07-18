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
mod forms;
mod pages;
mod state;
mod tables;
use self::state::{GitInfo, Page, Text};
pub use leptos::prelude::*;
pub use wasm_bindgen::JsCast;
#[derive(Clone, Debug, thiserror::Error)]
enum ApiError {
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
    waiters: Vec<futures::channel::oneshot::Sender<Result<(), ApiError>>>,
}
enum AuthRefreshWork {
    Start,
    Join(futures::channel::oneshot::Receiver<Result<(), ApiError>>),
}
#[derive(Clone, Copy)]
struct PageLoader {
    generation: RwSignal<u64>,
    notice: RwSignal<Option<Text>>,
    page: RwSignal<Page>,
}
impl PageLoader {
    fn new() -> Self {
        Self {
            generation: RwSignal::new(0u64),
            notice: RwSignal::new(None),
            page: RwSignal::new(Page::Loading),
        }
    }
    fn page(self) -> RwSignal<Page> {
        self.page
    }
    fn set(self, value: Page) {
        self.page.set(value);
    }
    fn set_notice(self, value: Text) {
        self.notice.set(Some(value));
    }
}
fn auth_refresh_state_error() -> ApiError {
    ApiError::Request(
        Text::try_from(str_constants::AUTHENTICATION_REFRESH_STATE_IS_UNAVAILABLE.to_owned())
            .unwrap_or_default(),
    )
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
    async fn get<Output>(
        &self,
        route: server_admin_contract::AdminRoute,
    ) -> Result<Output, ApiError>
    where
        Output: serde::de::DeserializeOwned,
    {
        let response = self.transport_response(route, Vec::new()).await?;
        serde_json::from_slice(response.body().as_ref()).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })
    }
    async fn get_table<Output>(
        &self,
        route: server_admin_contract::AdminRoute,
        state: &crate::table_state::TableState,
    ) -> Result<Output, ApiError>
    where
        Output: serde::de::DeserializeOwned,
    {
        let path = server_admin_contract::AdminRoutePath::try_from(format!(
            "{}?{}",
            route.path(),
            state.query()
        ))
        .map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })?;
        let response = self.transport_response_at(route, path, Vec::new()).await?;
        serde_json::from_slice(response.body().as_ref()).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })
    }
    async fn send_json<Input>(
        self,
        route: server_admin_contract::AdminRoute,
        input: Input,
    ) -> Result<(), ApiError>
    where
        Input: serde::Serialize,
    {
        let body = serde_json::to_vec(&input).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })?;
        let _response = self.transport_response(route, body).await?;
        Ok(())
    }
    async fn send_json_response<Input, Output>(
        &self,
        route: server_admin_contract::AdminRoute,
        input: Input,
    ) -> Result<Output, ApiError>
    where
        Input: serde::Serialize,
        Output: serde::de::DeserializeOwned,
    {
        let body = serde_json::to_vec(&input).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })?;
        let response = self.transport_response(route, body).await?;
        serde_json::from_slice(response.body().as_ref()).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })
    }
    async fn send(self, route: server_admin_contract::AdminRoute) -> Result<(), ApiError> {
        let _response = self.transport_response(route, Vec::new()).await?;
        Ok(())
    }
    async fn transport_response(
        &self,
        route: server_admin_contract::AdminRoute,
        body: Vec<u8>,
    ) -> Result<frontend_contract::TransportResponse, ApiError> {
        self.transport_response_at(route, route.path(), body).await
    }
    async fn transport_response_at(
        &self,
        route: server_admin_contract::AdminRoute,
        path: server_admin_contract::AdminRoutePath,
        body: Vec<u8>,
    ) -> Result<frontend_contract::TransportResponse, ApiError> {
        let response = self
            .transport_response_once(route, &path, body.as_slice())
            .await?;
        if u16::from(response.status()) == 401u16
            && !matches!(
                route,
                server_admin_contract::AdminRoute::Refresh
                    | server_admin_contract::AdminRoute::SignIn
            )
        {
            self.refresh_session().await?;
            return self
                .transport_response_once(route, &path, body.as_slice())
                .await
                .and_then(|retried| {
                    let expected = route.contract().success_status().transport_status();
                    if retried.status() == expected {
                        Ok(retried)
                    } else {
                        Err(response_error(
                            retried.status(),
                            retried.body(),
                            retried.retry_after(),
                        ))
                    }
                });
        }
        let expected = route.contract().success_status().transport_status();
        if response.status() != expected {
            return Err(response_error(
                response.status(),
                response.body(),
                response.retry_after(),
            ));
        }
        Ok(response)
    }
    async fn transport_response_once(
        &self,
        route: server_admin_contract::AdminRoute,
        path: &server_admin_contract::AdminRoutePath,
        body: &[u8],
    ) -> Result<frontend_contract::TransportResponse, ApiError> {
        Self::send_once(self.transport, route, path, body).await
    }
    async fn send_once(
        transport: crate::transport::GlooTransport,
        route: server_admin_contract::AdminRoute,
        path: &server_admin_contract::AdminRoutePath,
        body: &[u8],
    ) -> Result<frontend_contract::TransportResponse, ApiError> {
        let request = frontend_contract::TransportRequest::new(
            frontend_contract::TransportBody::from(body.to_vec()),
            frontend_contract::TransportPath::try_from(path.as_ref().to_owned()).map_err(
                |error| ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default()),
            )?,
            route.contract(),
        );
        frontend_contract::Transport::send(&transport, request)
            .await
            .map_err(|error| {
                ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
            })
    }
    async fn refresh_session(&self) -> Result<(), ApiError> {
        let now = crate::auth_keep_alive::StdAuthRefreshInstant::now();
        let work = {
            let mut coordinator = self
                .auth_refresh
                .write()
                .map_err(|_error| auth_refresh_state_error())?;
            match coordinator.state.begin(now) {
                crate::auth_keep_alive::AuthRefreshBegin::Start => AuthRefreshWork::Start,
                crate::auth_keep_alive::AuthRefreshBegin::Join => {
                    let (sender, receiver) = futures::channel::oneshot::channel();
                    coordinator.waiters.push(sender);
                    AuthRefreshWork::Join(receiver)
                }
                crate::auth_keep_alive::AuthRefreshBegin::Rejected => {
                    redirect(server_admin_contract::AdminFrontendPath::SignIn.get());
                    return Err(ApiError::Status(
                        401u16,
                        Text::try_from(str_constants::AUTHENTICATION_REFRESH_REJECTED.to_owned())
                            .unwrap_or_default(),
                    ));
                }
                crate::auth_keep_alive::AuthRefreshBegin::Wait => {
                    return Err(ApiError::Request(
                        Text::try_from(
                            str_constants::AUTHENTICATION_REFRESH_RETRY_IS_DELAYED.to_owned(),
                        )
                        .unwrap_or_default(),
                    ));
                }
            }
        };
        if let AuthRefreshWork::Join(receiver) = work {
            return receiver
                .await
                .map_err(|_error| auth_refresh_state_error())?;
        }
        let refresh_path = server_admin_contract::AdminRoute::Refresh.path();
        let response = Self::send_once(
            self.transport,
            server_admin_contract::AdminRoute::Refresh,
            &refresh_path,
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
                Err(response_error(
                    value.status(),
                    value.body(),
                    value.retry_after(),
                ))
            }
        });
        let outcome = match &result {
            Ok(()) => crate::auth_keep_alive::AuthRefreshOutcome::Refreshed,
            Err(ApiError::Status(401u16 | 403u16, _detail)) => {
                crate::auth_keep_alive::AuthRefreshOutcome::Rejected
            }
            Err(ApiError::Request(_) | ApiError::Status(_, _)) => {
                crate::auth_keep_alive::AuthRefreshOutcome::TemporaryFailure
            }
        };
        let waiters = {
            let mut coordinator = self
                .auth_refresh
                .write()
                .map_err(|_error| auth_refresh_state_error())?;
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
            redirect(server_admin_contract::AdminFrontendPath::SignIn.get());
        }
        result
    }
    async fn audit(&self) -> Result<server_admin_contract::AdminAuditPage, ApiError> {
        let route = server_admin_contract::AdminRoute::Audit;
        let query = search_query();
        if query.is_empty() {
            return self.get(route).await;
        }
        let path =
            server_admin_contract::AdminRoutePath::try_from(format!("{}{}", route.path(), query))
                .map_err(|error| {
                ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
            })?;
        let response = self.transport_response_at(route, path, Vec::new()).await?;
        serde_json::from_slice(response.body().as_ref()).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })
    }
    async fn audit_export(&self) -> Result<server_admin_contract::AdminAuditExport, ApiError> {
        let route = server_admin_contract::AdminRoute::AuditExport;
        let path = server_admin_contract::AdminRoutePath::try_from(format!(
            "{}{}",
            route.path(),
            search_query()
        ))
        .map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })?;
        let response = self.transport_response_at(route, path, Vec::new()).await?;
        serde_json::from_slice(response.body().as_ref()).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })
    }
    async fn me(&self) -> Result<server_admin_contract::AuthenticatedAdmin, ApiError> {
        self.get(server_admin_contract::AdminRoute::Me).await
    }
    async fn branding(&self) -> Result<server_admin_contract::AdminBrandingView, ApiError> {
        self.get(server_admin_contract::AdminRoute::Branding).await
    }
    async fn dashboard(&self) -> Result<server_admin_contract::AdminDashboardView, ApiError> {
        self.get(server_admin_contract::AdminRoute::Dashboard).await
    }
    async fn mfa_status(&self) -> Result<server_admin_contract::AdminMfaStatus, ApiError> {
        self.get(server_admin_contract::AdminRoute::MfaStatus).await
    }
    async fn permissions(
        &self,
        state: &crate::table_state::TableState,
    ) -> Result<server_admin_contract::AdminPermissionsPage, ApiError> {
        self.get_table(server_admin_contract::AdminRoute::Permissions, state)
            .await
    }
    async fn roles(
        &self,
        state: &crate::table_state::TableState,
    ) -> Result<server_admin_contract::AdminRolesPage, ApiError> {
        self.get_table(server_admin_contract::AdminRoute::Roles, state)
            .await
    }
    async fn settings(&self) -> Result<server_admin_contract::AdminSettingsView, ApiError> {
        self.get(server_admin_contract::AdminRoute::Settings).await
    }
    async fn sessions(&self) -> Result<Vec<server_admin_contract::AdminSessionView>, ApiError> {
        self.get(server_admin_contract::AdminRoute::Sessions).await
    }
    async fn users(
        &self,
        state: &crate::table_state::TableState,
    ) -> Result<server_admin_contract::AdminUsersPage, ApiError> {
        self.get_table(server_admin_contract::AdminRoute::Users, state)
            .await
    }
    async fn revoke_session(
        self,
        session_id: server_admin_contract::AdminSessionIdentifier,
    ) -> Result<(), ApiError> {
        let suffix = frontend_contract::typed_parameterized_route_path::<
            server_admin_contract::AdminRevokeSessionRoute,
        >(&session_id);
        let path = server_admin_contract::AdminRoutePath::try_from(format!(
            "{}{}{}",
            str_constants::API_V1,
            server_admin_contract::AdminFrontendPath::Root.get(),
            String::from(suffix)
        ))
        .map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })?;
        let _response = self
            .transport_response_at(
                server_admin_contract::AdminRoute::RevokeSession,
                path,
                Vec::new(),
            )
            .await?;
        Ok(())
    }
    async fn metrics(&self) -> Result<Text, ApiError> {
        let route = server_admin_contract::AdminRoute::Metrics;
        let response = self.transport_response(route, Vec::new()).await?;
        String::from_utf8(response.body().as_ref().to_vec())
            .map(|value| Text::try_from(value).unwrap_or_default())
            .map_err(|error| {
                ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
            })
    }
    async fn open_api(&self) -> Result<Text, ApiError> {
        let document = self
            .get::<serde_json::Value>(server_admin_contract::AdminRoute::OpenApi)
            .await?;
        serde_json::to_string_pretty(&document)
            .map(|value| Text::try_from(value).unwrap_or_default())
            .map_err(|error| {
                ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
            })
    }
    async fn version(&self) -> Result<GitInfo, ApiError> {
        self.get(server_admin_contract::AdminRoute::Version).await
    }
    async fn sign_in(
        &self,
        input: server_admin_contract::AdminSignInReq,
    ) -> Result<server_admin_contract::AdminSignInRes, ApiError> {
        let route = server_admin_contract::AdminRoute::SignIn;
        let body = serde_json::to_vec(&input).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })?;
        let response = self.transport_response(route, body).await?;
        serde_json::from_slice(response.body().as_ref()).map_err(|error| {
            ApiError::Request(Text::try_from(error.to_string()).unwrap_or_default())
        })
    }
}
fn response_error(
    status: frontend_contract::TransportStatus,
    body: &frontend_contract::TransportBody,
    retry_after: Option<&frontend_contract::TransportRetryAfter>,
) -> ApiError {
    let mut detail = frontend_contract::decode_api_problem(body).map_or_else(
        || Text::try_from(str_constants::REQUEST_FAILED.to_owned()).unwrap_or_default(),
        |problem| Text::try_from(problem.detail().as_ref().to_owned()).unwrap_or_default(),
    );
    if u16::from(status) == 429u16
        && let Some(retry_after) = retry_after
    {
        detail = Text::try_from(format!("{} Retry after: {}.", detail, retry_after.as_ref()))
            .unwrap_or(detail);
    }
    ApiError::Status(u16::from(status), detail)
}
fn browser_window() -> Option<web_sys::Window> {
    web_sys::window()
}
fn path() -> String {
    browser_window()
        .and_then(|value| value.location().pathname().ok())
        .unwrap_or_else(|| {
            server_admin_contract::AdminFrontendPath::Root
                .get()
                .to_owned()
        })
}
fn search_query() -> String {
    browser_window()
        .and_then(|value| value.location().search().ok())
        .unwrap_or_default()
}
fn query_value(key: &str) -> String {
    search_query()
        .trim_start_matches('?')
        .split('&')
        .filter_map(|part| part.split_once('='))
        .find_map(|(candidate, value)| {
            (candidate == key)
                .then(|| crate::table_state::percent_decode(value))
                .flatten()
        })
        .unwrap_or_default()
}
fn table_state(
    default_sort: server_admin_contract::AdminTableSortField,
    options: &[server_admin_contract::AdminTableSortField],
) -> crate::table_state::TableState {
    crate::table_state::TableState::from_query(default_sort, options, search_query().as_str())
}
fn apply_table_state_url(
    page: server_admin_contract::AdminPage,
    state: &crate::table_state::TableState,
) {
    replace_path(format!("{}?{}", page.path(), state.query()).as_str());
}
fn redirect(path: &str) {
    if let Some(value) = browser_window() {
        let _result = value.location().set_href(path);
    }
}
fn reload() {
    if let Some(value) = browser_window() {
        let _result = value.location().reload();
    }
}
fn apply_branding(value: &server_admin_contract::AdminBrandingView) {
    let Some(window) = browser_window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    document.set_title(
        value
            .tab_title()
            .map_or_else(|| value.site_name().as_ref(), AsRef::<str>::as_ref),
    );
    if let Some(root) = document.document_element()
        && let Some(html_root) = JsCast::dyn_ref::<web_sys::HtmlElement>(&root)
    {
        let color = value
            .primary_color()
            .map_or(str_constants::PRIMARY_COLOR_DEFAULT, AsRef::<str>::as_ref);
        let _result = html_root
            .style()
            .set_property(str_constants::PRIMARY_CSS_VARIABLE, color);
    }
}
fn push_path(path: &str) {
    if let Some(value) = browser_window()
        && let Ok(history) = value.history()
    {
        let _result = history.push_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            str_constants::PG_CRUD_EMPTY_SQL_SUFFIX,
            Some(path),
        );
    }
}
fn replace_path(path: &str) {
    if let Some(value) = browser_window()
        && let Ok(history) = value.history()
    {
        let _result = history.replace_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            str_constants::PG_CRUD_EMPTY_SQL_SUFFIX,
            Some(path),
        );
    }
}
fn authentication_is_rejected(error: &ApiError) -> bool {
    matches!(error, ApiError::Status(401u16 | 403u16, _detail))
}
fn load(client: AdminApiClient, loader: PageLoader) {
    let generation = loader.generation.get_untracked().saturating_add(1u64);
    loader.generation.set(generation);
    loader.set(Page::Loading);
    leptos::task::spawn_local(async move {
        let current_path = path();
        let current_page = server_admin_contract::AdminPage::from_path(
            server_admin_contract::AdminPagePathRef::from(current_path.as_str()),
        );
        let result = match current_page {
            Some(server_admin_contract::AdminPage::Dashboard) => {
                client.dashboard().await.map(Page::Dashboard)
            }
            Some(server_admin_contract::AdminPage::Users) => client
                .users(&table_state(
                    server_admin_contract::AdminTableSortField::UserLogin,
                    &server_admin_contract::AdminTableSortField::USER,
                ))
                .await
                .map(|page| {
                    Page::Users(page.items().to_vec(), page.roles().to_vec(), page.total())
                }),
            Some(server_admin_contract::AdminPage::Roles) => client
                .roles(&table_state(
                    server_admin_contract::AdminTableSortField::RoleName,
                    &server_admin_contract::AdminTableSortField::ROLE,
                ))
                .await
                .map(|page| {
                    Page::Roles(
                        page.items().to_vec(),
                        page.permissions().to_vec(),
                        page.total(),
                    )
                }),
            Some(server_admin_contract::AdminPage::Permissions) => client
                .permissions(&table_state(
                    server_admin_contract::AdminTableSortField::PermissionName,
                    &server_admin_contract::AdminTableSortField::PERMISSION,
                ))
                .await
                .map(|page| Page::Permissions(page.items().to_vec(), page.total())),
            Some(server_admin_contract::AdminPage::Profile) => {
                client.mfa_status().await.map(Page::Profile)
            }
            Some(server_admin_contract::AdminPage::Audit) => client
                .audit()
                .await
                .map(|page| Page::Audit(page.items().to_vec(), page.next_cursor().cloned())),
            Some(server_admin_contract::AdminPage::Settings) => {
                client.settings().await.map(Page::Settings)
            }
            Some(server_admin_contract::AdminPage::Sessions) => {
                client.sessions().await.map(Page::Sessions)
            }
            Some(server_admin_contract::AdminPage::Metrics) => {
                client.metrics().await.map(Page::Text)
            }
            Some(server_admin_contract::AdminPage::Version) => {
                client.version().await.map(|value| {
                    Page::Text(value.commit.unwrap_or_else(|| {
                        Text::try_from(str_constants::UNKNOWN_VERSION.to_owned())
                            .unwrap_or_default()
                    }))
                })
            }
            Some(server_admin_contract::AdminPage::OpenApi) => {
                client.open_api().await.map(Page::OpenApi)
            }
            None => {
                redirect(server_admin_contract::AdminPage::Version.path().as_ref());
                return;
            }
        };
        if loader.generation.get_untracked() == generation {
            loader.set(result.unwrap_or_else(|error| {
                Page::Error(Text::try_from(error.to_string()).unwrap_or_default())
            }));
        }
    });
}
fn run_action<FutureValue>(future: FutureValue, client: AdminApiClient, loader: PageLoader)
where
    FutureValue: Future<Output = Result<(), ApiError>> + 'static,
{
    leptos::task::spawn_local(async move {
        match future.await {
            Ok(()) => load(client, loader),
            Err(error) => loader.set(Page::Error(
                Text::try_from(error.to_string()).unwrap_or_default(),
            )),
        }
    });
}
#[component]
pub fn App() -> impl IntoView {
    let client = AdminApiClient::new();
    let branding = LocalResource::new({
        let client = client.clone();
        move || {
            let client = client.clone();
            async move { client.branding().await }
        }
    });
    if path() == server_admin_contract::AdminFrontendPath::SignIn.get() {
        return view! { <Suspense fallback=move || view! { <main><p>"Loading..."</p></main> }>{move || { let client = client.clone(); Suspend::new(async move { let branding = branding.await.ok(); if let Some(value) = branding.as_ref() { apply_branding(value); } view! { <forms::SignIn client branding /> } }) }}</Suspense> }.into_any();
    }
    let auth = LocalResource::new({
        let client = client.clone();
        move || {
            let client = client.clone();
            async move { client.me().await }
        }
    });
    let client_for_auth = client.clone();
    view! { <Suspense fallback=move || view! { <main><p>"Loading..."</p></main> }>{move || { let client = client_for_auth.clone(); Suspend::new(async move { let branding = branding.await.ok(); if let Some(value) = branding.as_ref() { apply_branding(value); } match auth.await { Ok(value) => view! { <pages::Shell auth=value client=client.clone() branding /> }.into_any(), Err(error) if authentication_is_rejected(&error) => { redirect(server_admin_contract::AdminFrontendPath::SignIn.get()); view! { <main></main> }.into_any() }, Err(error) => view! { <main class="auth-page"><section class="auth-card"><div class="alert error" role="alert"><strong>"Unable to verify session"</strong><span>{error.to_string()}</span></div><button class="primary-button" type="button" on:click=move |_| reload()>"Try again"</button></section></main> }.into_any() } }) }}</Suspense> }.into_any()
}
