#[path = "application_html_actions_auth.rs"]
mod auth;
#[path = "application_html_actions_roles.rs"]
mod roles;
#[path = "application_html_actions_sessions.rs"]
mod sessions;
#[path = "application_html_actions_settings.rs"]
mod settings;
#[path = "application_html_actions_users.rs"]
mod users;

#[frontend_contract::domain_types::route_operation]
pub(super) async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::domain_types::AdminFrontendPath::Users.get(),
    ))
}

async fn assignment_action<Ids, Parse, Request, BuildRequest, Target, Run, RunFuture>(
    auth: super::super::AdminAuthReq,
    expected: &super::forms::AdminHtmlFormText,
    selected: super::forms::StdAdminHtmlSelected,
    parse: Parse,
    path: server_admin_contract::domain_types::AdminFrontendPath,
    build_request: BuildRequest,
    target: Target,
    run: Run,
) -> axum::response::Response
where
    Parse: Fn(&super::forms::AdminHtmlFormText) -> Result<Ids, super::super::AdminError>,
    BuildRequest: FnOnce(Ids, Ids) -> Request,
    Run: FnOnce(
        super::super::AdminAuthReq,
        Target,
        super::super::AxumAdminJson<Request>,
    ) -> RunFuture,
    RunFuture: Future<Output = Result<super::super::AxumAdminResponse, super::super::AdminError>>,
{
    let (auth, expected, selected) =
        match super::authenticated_selected_form_impl::authenticated_selected_form_impl(
            auth, expected, selected, parse,
        ) {
            Ok(values) => values,
            Err(error) => return axum::response::IntoResponse::into_response(error),
        };
    super::action_result_impl::action_result_impl(
        run(
            auth,
            target,
            super::super::AxumAdminJson(build_request(expected, selected)),
        )
        .await,
        path,
    )
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::Root, root),
)]
struct AdminHtmlActionRouteRegistry;

pub(super) fn router() -> super::super::AxumAdminStateRouter {
    super::super::AxumAdminStateRouter::from(
        AdminHtmlActionRouteRegistry::router()
            .merge(axum::Router::from(auth::router()))
            .merge(axum::Router::from(roles::router()))
            .merge(axum::Router::from(sessions::router()))
            .merge(axum::Router::from(settings::router()))
            .merge(axum::Router::from(users::router())),
    )
}
