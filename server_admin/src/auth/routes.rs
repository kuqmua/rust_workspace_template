#![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition
macro_rules! admin_route_registry {
    ($callback:ident) => {
        $callback! {
            (server_admin_contract::AdminSignInRoute, super::sign_in),
            (server_admin_contract::AdminRefreshRoute, super::refresh),
            (server_admin_contract::AdminSignOutRoute, super::sign_out),
            (server_admin_contract::AdminMeRoute, super::me),
            (server_admin_contract::AdminSessionsRoute, super::sessions),
            (server_admin_contract::AdminRevokeSessionRoute, super::revoke_session),
            (server_admin_contract::AdminRevokeAllSessionsRoute, super::revoke_all_sessions),
            (server_admin_contract::AdminListUsersRoute, super::list_users),
            (server_admin_contract::AdminCreateUserRoute, super::create_user),
            (server_admin_contract::AdminUpdateUserRoute, super::update_user),
            (server_admin_contract::AdminDeleteUserRoute, super::delete_user),
            (server_admin_contract::AdminSetUserPasswordRoute, super::set_user_password),
            (server_admin_contract::AdminSetUserBanRoute, super::set_user_ban),
            (server_admin_contract::AdminSetUserRolesRoute, super::set_user_roles),
            (server_admin_contract::AdminListRolesRoute, super::list_roles),
            (server_admin_contract::AdminCreateRoleRoute, super::create_role),
            (server_admin_contract::AdminUpdateRoleRoute, super::update_role),
            (server_admin_contract::AdminDeleteRoleRoute, super::delete_role),
            (server_admin_contract::AdminSetRolePermissionsRoute, super::set_role_permissions),
            (server_admin_contract::AdminListPermissionsRoute, super::list_permissions),
            (server_admin_contract::AdminAuditLogRoute, super::audit_log),
            (server_admin_contract::AdminSettingsRoute, super::settings),
            (server_admin_contract::AdminUpdateSettingsRoute, super::update_settings),
        }
    };
}
macro_rules! declare_admin_open_api {
    ($(($route:path, $handler:path)),+ $(,)?) => {
#[allow(clippy::needless_for_each)] // utoipa 4 generated component registration uses iterator callbacks
#[derive(utoipa::OpenApi)]
#[openapi(
    paths($($handler),+),
    components(schemas(server_admin_contract::AdminSignInReq, server_admin_contract::AdminSignInRes, server_admin_contract::AuthenticatedAdmin, server_admin_contract::AdminSessionView, server_admin_contract::AdminSessionTimestamp, server_admin_contract::AdminSessionIdentifier, frontend_contract::ApiProblem, server_admin_contract::AdminApiErrorCode, server_admin_contract::AdminApiErrorBody, server_admin_contract::AdminText, server_admin_contract::AdminBool, server_admin_contract::AdminPermissionValue, server_admin_contract::AdminCreateUserReq, server_admin_contract::AdminCreateUserRes, server_admin_contract::AdminUpdateUserReq, server_admin_contract::AdminSetUserPasswordReq, server_admin_contract::AdminSetUserBanReq, server_admin_contract::AdminSetUserRolesReq, server_admin_contract::AdminCreateRoleReq, server_admin_contract::AdminCreateRoleRes, server_admin_contract::AdminUpdateRoleReq, server_admin_contract::AdminSetRolePermissionsReq, server_admin_contract::AdminAuditView, server_admin_contract::AdminAuditTimestamp, server_admin_contract::SerdeJsonAdminAuditDetails, server_admin_contract::AdminUpdateSettingsReq, server_admin_contract::AdminSettingText, server_admin_contract::AdminUserSummary, server_admin_contract::AdminRoleSummary, server_admin_contract::AdminPermissionSummary, server_admin_contract::AdminSettingsView, crate::UuidAdminValue, crate::AdminPassword, crate::AdminLogin, crate::AdminDisplayName, crate::AdminRoleName, crate::AdminUserId, crate::AdminRoleId, crate::AdminPermissionId, crate::AdminPermission, crate::AdminSessionId, crate::AdminAuditLogId, crate::AdminAuditAction, crate::AdminAuditResource)),
    tags((name = "admin_auth", description = "Administrator authentication and sessions"), (name = "admin_users", description = "Administrator user security operations"), (name = "admin_roles", description = "Administrator role security operations"), (name = "admin_audit", description = "Administrator audit log"), (name = "admin_settings", description = "Administrator system settings"))
)]
struct AdminAuthOpenApi;
    };
}
admin_route_registry!(declare_admin_open_api);
pub(super) fn open_api() -> super::UtoipaAdminAuthOpenApi {
    let mut document = <AdminAuthOpenApi as utoipa::OpenApi>::openapi();
    document
        .paths
        .paths
        .values_mut()
        .flat_map(|path| path.operations.values_mut())
        .for_each(|operation| {
            let response = operation
                .responses
                .responses
                .entry(str_constants::VALUE_429.to_owned())
                .or_insert_with(|| {
                    utoipa::openapi::RefOr::T(utoipa::openapi::response::Response::new(
                        str_constants::REQUEST_RATE_LIMIT_EXCEEDED,
                    ))
                });
            if let utoipa::openapi::RefOr::T(response_value) = response {
                let _previous_header = response_value.headers.insert(
                    str_constants::RETRY_AFTER.to_owned(),
                    utoipa::openapi::header::Header::default(),
                );
            }
        });
    if let Some(components) = document.components.as_mut() {
        components.add_security_scheme(
            str_constants::ADMIN_COOKIE,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        str_constants::SERVER_ADMIN_ACCESS_COOKIE_NAME,
                        str_constants::HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE,
                    ),
                ),
            ),
        );
        components.add_security_scheme(
            str_constants::ADMIN_CSRF,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        str_constants::X_CSRF_TOKEN,
                        str_constants::CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION,
                    ),
                ),
            ),
        );
    }
    super::UtoipaAdminAuthOpenApi(document)
}
pub(super) fn routes(state: super::StdSharedAdminAuthSvcState) -> super::AxumAdminAuthRouter {
    macro_rules! build_admin_router {
        ($(($route:path, $handler:path)),+ $(,)?) => {
            axum::Router::new()
                $(.route(
                    frontend_contract::typed_route_path::<$route>().as_ref(),
                    typed_method_router::<$route, _, _, _>($handler),
                ))+
        };
    }
    let base_router = admin_route_registry!(build_admin_router)
        .method_not_allowed_fallback(async || super::AdminApiError::MethodNotAllowed);
    let router = match <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    super::AxumAdminAuthRouter(router.with_state(state))
}
fn typed_method_router<Route, Handler, HandlerArguments, State>(
    handler: Handler,
) -> axum::routing::MethodRouter<State>
where
    Route: frontend_contract::TypedRoute,
    Handler: axum::handler::Handler<HandlerArguments, State>,
    HandlerArguments: 'static,
    State: Clone + Send + Sync + 'static,
{
    let filter = match <Route as frontend_contract::TypedRoute>::metadata().route_method() {
        frontend_contract::RouteMethod::Connect => axum::routing::MethodFilter::CONNECT,
        frontend_contract::RouteMethod::Delete => axum::routing::MethodFilter::DELETE,
        frontend_contract::RouteMethod::Get => axum::routing::MethodFilter::GET,
        frontend_contract::RouteMethod::Head => axum::routing::MethodFilter::HEAD,
        frontend_contract::RouteMethod::Options => axum::routing::MethodFilter::OPTIONS,
        frontend_contract::RouteMethod::Patch => axum::routing::MethodFilter::PATCH,
        frontend_contract::RouteMethod::Post => axum::routing::MethodFilter::POST,
        frontend_contract::RouteMethod::Put => axum::routing::MethodFilter::PUT,
        frontend_contract::RouteMethod::Trace => axum::routing::MethodFilter::TRACE,
    };
    axum::routing::on(filter, handler)
}
