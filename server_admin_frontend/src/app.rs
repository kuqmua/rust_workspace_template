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
    page: RwSignal<Page>,
}
impl PageLoader {
    fn new() -> Self {
        Self {
            generation: RwSignal::new(0u64),
            page: RwSignal::new(Page::Loading),
        }
    }
    fn page(self) -> RwSignal<Page> {
        self.page
    }
    fn set(self, value: Page) {
        self.page.set(value);
    }
}
fn auth_refresh_state_error() -> ApiError {
    ApiError::Request(Text::from(
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
    async fn get<Output>(
        &self,
        route: server_admin_contract::AdminRoute,
    ) -> Result<Output, ApiError>
    where
        Output: serde::de::DeserializeOwned,
    {
        let response = self.transport_response(route, Vec::new()).await?;
        serde_json::from_slice(response.body().as_ref())
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))
    }
    async fn send_json<Input>(
        self,
        route: server_admin_contract::AdminRoute,
        input: Input,
    ) -> Result<(), ApiError>
    where
        Input: serde::Serialize,
    {
        let body = serde_json::to_vec(&input)
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))?;
        let _response = self.transport_response(route, body).await?;
        Ok(())
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
                        Err(response_error(retried.status(), retried.body()))
                    }
                });
        }
        let expected = route.contract().success_status().transport_status();
        if response.status() != expected {
            return Err(response_error(response.status(), response.body()));
        }
        Ok(response)
    }
    async fn transport_response_once(
        &self,
        route: server_admin_contract::AdminRoute,
        body: &[u8],
    ) -> Result<frontend_contract::TransportResponse, ApiError> {
        Self::send_once(self.transport, route, body).await
    }
    async fn send_once(
        transport: crate::transport::GlooTransport,
        route: server_admin_contract::AdminRoute,
        body: &[u8],
    ) -> Result<frontend_contract::TransportResponse, ApiError> {
        let path = route.path();
        let request = frontend_contract::TransportRequest::new(
            frontend_contract::TransportBody::from(body.to_vec()),
            frontend_contract::TransportPath::try_from(path.as_ref().to_owned())
                .map_err(|error| ApiError::Request(Text::from(error.to_string())))?,
            route.contract(),
        );
        frontend_contract::Transport::send(&transport, request)
            .await
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))
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
                    redirect(str_constants::admin_page_paths::SIGN_IN);
                    return Err(ApiError::Status(
                        401u16,
                        Text::from("authentication refresh rejected".to_owned()),
                    ));
                }
                crate::auth_keep_alive::AuthRefreshBegin::Wait => {
                    return Err(ApiError::Request(Text::from(
                        "authentication refresh retry is delayed".to_owned(),
                    )));
                }
            }
        };
        if let AuthRefreshWork::Join(receiver) = work {
            return receiver
                .await
                .map_err(|_error| auth_refresh_state_error())?;
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
                Err(response_error(value.status(), value.body()))
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
            redirect(str_constants::admin_page_paths::SIGN_IN);
        }
        result
    }
    async fn audit(&self) -> Result<Vec<server_admin_contract::AdminAuditView>, ApiError> {
        self.get(server_admin_contract::AdminRoute::Audit).await
    }
    async fn me(&self) -> Result<server_admin_contract::AuthenticatedAdmin, ApiError> {
        self.get(server_admin_contract::AdminRoute::Me).await
    }
    async fn permissions(
        &self,
    ) -> Result<Vec<server_admin_contract::AdminPermissionSummary>, ApiError> {
        self.get(server_admin_contract::AdminRoute::Permissions)
            .await
    }
    async fn roles(&self) -> Result<Vec<server_admin_contract::AdminRoleSummary>, ApiError> {
        self.get(server_admin_contract::AdminRoute::Roles).await
    }
    async fn settings(&self) -> Result<server_admin_contract::AdminSettingsView, ApiError> {
        self.get(server_admin_contract::AdminRoute::Settings).await
    }
    async fn users(&self) -> Result<Vec<server_admin_contract::AdminUserSummary>, ApiError> {
        self.get(server_admin_contract::AdminRoute::Users).await
    }
    async fn metrics(&self) -> Result<Text, ApiError> {
        let route = server_admin_contract::AdminRoute::Metrics;
        let response = self.transport_response(route, Vec::new()).await?;
        String::from_utf8(response.body().as_ref().to_vec())
            .map(Text::from)
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))
    }
    async fn open_api(&self) -> Result<Text, ApiError> {
        let document = self
            .get::<serde_json::Value>(server_admin_contract::AdminRoute::OpenApi)
            .await?;
        serde_json::to_string_pretty(&document)
            .map(Text::from)
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))
    }
    async fn version(&self) -> Result<GitInfo, ApiError> {
        self.get(server_admin_contract::AdminRoute::Version).await
    }
    async fn sign_in(
        &self,
        input: server_admin_contract::AdminSignInReq,
    ) -> Result<server_admin_contract::AdminSignInRes, ApiError> {
        let route = server_admin_contract::AdminRoute::SignIn;
        let body = serde_json::to_vec(&input)
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))?;
        let response = self.transport_response(route, body).await?;
        serde_json::from_slice(response.body().as_ref())
            .map_err(|error| ApiError::Request(Text::from(error.to_string())))
    }
}
fn response_error(
    status: frontend_contract::TransportStatus,
    body: &frontend_contract::TransportBody,
) -> ApiError {
    let detail = frontend_contract::decode_api_problem(body).map_or_else(
        || Text::from("request failed".to_owned()),
        |problem| Text::from(problem.detail().as_ref().to_owned()),
    );
    ApiError::Status(u16::from(status), detail)
}
fn browser_window() -> Option<web_sys::Window> {
    web_sys::window()
}
fn path() -> String {
    browser_window()
        .and_then(|value| value.location().pathname().ok())
        .unwrap_or_else(|| str_constants::admin_page_paths::ROOT.to_owned())
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
fn push_path(path: &str) {
    if let Some(value) = browser_window()
        && let Ok(history) = value.history()
    {
        let _result = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path));
    }
}
fn replace_path(path: &str) {
    if let Some(value) = browser_window()
        && let Ok(history) = value.history()
    {
        let _result = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path));
    }
}
fn authentication_is_rejected(error: &ApiError) -> bool {
    matches!(error, ApiError::Status(401u16 | 403u16, _detail))
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
            Some(server_admin_contract::AdminPage::OpenApi) => {
                client.open_api().await.map(Page::OpenApi)
            }
            None => {
                redirect(server_admin_contract::AdminPage::Version.path().as_ref());
                return;
            }
        };
        if loader.generation.get_untracked() == generation {
            loader.set(result.unwrap_or_else(|error| Page::Error(Text::from(error.to_string()))));
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
            Err(error) => loader.set(Page::Error(Text::from(error.to_string()))),
        }
    });
}
#[component]
pub fn App() -> impl IntoView {
    let client = AdminApiClient::new();
    if path() == str_constants::admin_page_paths::SIGN_IN {
        return view! { <forms::SignIn client /> }.into_any();
    }
    let auth = LocalResource::new({
        let client = client.clone();
        move || {
            let client = client.clone();
            async move { client.me().await }
        }
    });
    let client_for_auth = client.clone();
    view! { <Suspense fallback=move || view! { <main><p>"Loading..."</p></main> }>{move || { let client = client_for_auth.clone(); Suspend::new(async move { match auth.await { Ok(value) => view! { <pages::Shell auth=value client=client.clone() /> }.into_any(), Err(error) if authentication_is_rejected(&error) => { redirect(str_constants::admin_page_paths::SIGN_IN); view! { <main></main> }.into_any() }, Err(error) => view! { <main class="auth-page"><section class="auth-card"><div class="alert error" role="alert"><strong>"Unable to verify session"</strong><span>{error.to_string()}</span></div><button class="primary-button" type="button" on:click=move |_| reload()>"Try again"</button></section></main> }.into_any() } }) }}</Suspense> }.into_any()
}
