#![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition
#[allow(clippy::needless_for_each)] // utoipa 4 generated component registration uses iterator callbacks
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(super::sign_in, super::refresh, super::sign_out, super::me, super::sessions, super::revoke_session, super::revoke_all_sessions, super::list_users, super::create_user, super::update_user, super::set_user_password, super::set_user_ban, super::delete_user, super::set_user_roles, super::list_roles, super::create_role, super::update_role, super::delete_role, super::set_role_permissions, super::list_permissions, super::audit_log, super::settings, super::update_settings),
    components(schemas(server_admin_contract::AdminSignInReq, server_admin_contract::AdminSignInRes, server_admin_contract::AuthenticatedAdmin, server_admin_contract::AdminSessionView, server_admin_contract::AdminSessionTimestamp, server_admin_contract::AdminSessionIdentifier, frontend_contract::ApiProblem, server_admin_contract::AdminApiErrorCode, server_admin_contract::AdminApiErrorBody, server_admin_contract::AdminText, server_admin_contract::AdminBool, server_admin_contract::AdminPermissionValue, server_admin_contract::AdminCreateUserReq, server_admin_contract::AdminCreateUserRes, server_admin_contract::AdminUpdateUserReq, server_admin_contract::AdminSetUserPasswordReq, server_admin_contract::AdminSetUserBanReq, server_admin_contract::AdminSetUserRolesReq, server_admin_contract::AdminCreateRoleReq, server_admin_contract::AdminCreateRoleRes, server_admin_contract::AdminUpdateRoleReq, server_admin_contract::AdminSetRolePermissionsReq, server_admin_contract::AdminAuditView, server_admin_contract::AdminAuditTimestamp, server_admin_contract::SerdeJsonAdminAuditDetails, server_admin_contract::AdminUpdateSettingsReq, server_admin_contract::AdminSettingText, server_admin_contract::AdminUserSummary, server_admin_contract::AdminRoleSummary, server_admin_contract::AdminPermissionSummary, server_admin_contract::AdminSettingsView, crate::UuidAdminValue, crate::AdminPassword, crate::AdminLogin, crate::AdminDisplayName, crate::AdminRoleName, crate::AdminUserId, crate::AdminRoleId, crate::AdminPermissionId, crate::AdminPermission, crate::AdminSessionId, crate::AdminAuditLogId, crate::AdminAuditAction, crate::AdminAuditResource)),
    tags((name = "admin_auth", description = "Administrator authentication and sessions"), (name = "admin_users", description = "Administrator user security operations"), (name = "admin_roles", description = "Administrator role security operations"), (name = "admin_audit", description = "Administrator audit log"), (name = "admin_settings", description = "Administrator system settings"))
)]
struct AdminAuthOpenApi;
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
    super::AxumAdminAuthRouter(
        axum::Router::new()
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>().as_ref(),
                axum::routing::post(super::sign_in),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminRefreshRoute>().as_ref(),
                axum::routing::post(super::refresh),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignOutRoute>().as_ref(),
                axum::routing::post(super::sign_out),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>().as_ref(),
                axum::routing::get(super::me),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSessionsRoute>().as_ref(),
                axum::routing::get(super::sessions).delete(super::revoke_all_sessions),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminRevokeSessionRoute>().as_ref(),
                axum::routing::delete(super::revoke_session),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>().as_ref(),
                axum::routing::get(super::list_users).post(super::create_user),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminUpdateUserRoute>().as_ref(),
                axum::routing::patch(super::update_user).delete(super::delete_user),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSetUserPasswordRoute>().as_ref(),
                axum::routing::post(super::set_user_password),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSetUserBanRoute>().as_ref(),
                axum::routing::post(super::set_user_ban),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListRolesRoute>().as_ref(),
                axum::routing::get(super::list_roles).post(super::create_role),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminUpdateRoleRoute>().as_ref(),
                axum::routing::patch(super::update_role).delete(super::delete_role),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSetRolePermissionsRoute>().as_ref(),
                axum::routing::put(super::set_role_permissions),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSetUserRolesRoute>().as_ref(),
                axum::routing::put(super::set_user_roles),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListPermissionsRoute>().as_ref(),
                axum::routing::get(super::list_permissions),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminAuditLogRoute>().as_ref(),
                axum::routing::get(super::audit_log),
            )
            .route(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSettingsRoute>().as_ref(),
                axum::routing::get(super::settings).patch(super::update_settings),
            )
            .method_not_allowed_fallback(async || super::AdminApiError::MethodNotAllowed)
            .layer(axum::extract::DefaultBodyLimit::max(65_536usize))
            .with_state(state),
    )
}
