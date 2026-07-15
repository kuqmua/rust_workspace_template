#![allow(clippy::single_call_fn)] // public facade keeps stable auth module paths while this module owns router and OpenAPI composition
#[allow(clippy::needless_for_each)] // utoipa 4 generated component registration uses iterator callbacks
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(super::sign_in, super::refresh, super::sign_out, super::me, super::sessions, super::revoke_session, super::revoke_all_sessions, super::list_users, super::create_user, super::update_user, super::set_user_password, super::set_user_ban, super::delete_user, super::set_user_roles, super::list_roles, super::create_role, super::update_role, super::delete_role, super::set_role_permissions, super::list_permissions, super::audit_log, super::settings, super::update_settings),
    components(schemas(server_admin_contract::AdminSignInReq, server_admin_contract::AdminSignInRes, server_admin_contract::AuthenticatedAdmin, super::AdminSessionView, frontend_contract::ApiProblem, server_admin_contract::AdminApiErrorCode, server_admin_contract::AdminCreateUserReq, server_admin_contract::AdminCreateUserRes, server_admin_contract::AdminUpdateUserReq, server_admin_contract::AdminSetUserPasswordReq, server_admin_contract::AdminSetUserBanReq, server_admin_contract::AdminSetUserRolesReq, server_admin_contract::AdminCreateRoleReq, server_admin_contract::AdminCreateRoleRes, server_admin_contract::AdminUpdateRoleReq, server_admin_contract::AdminSetRolePermissionsReq, server_admin_contract::AdminAuditView, server_admin_contract::AdminAuditTimestamp, server_admin_contract::SerdeJsonAdminAuditDetails, server_admin_contract::AdminUpdateSettingsReq, server_admin_contract::AdminSettingText, server_admin_contract::AdminUserSummary, server_admin_contract::AdminRoleSummary, server_admin_contract::AdminPermissionSummary, server_admin_contract::AdminSettingsView, crate::AdminPassword, crate::AdminLogin, crate::AdminDisplayName, crate::AdminRoleName, crate::AdminUserId, crate::AdminRoleId, crate::AdminPermissionId, crate::AdminPermission, crate::AdminSessionId, crate::AdminAuditLogId, crate::AdminAuditAction, crate::AdminAuditResource)),
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
                .entry(str_constants::text::VALUE_429.to_owned())
                .or_insert_with(|| {
                    utoipa::openapi::RefOr::T(utoipa::openapi::response::Response::new(
                        str_constants::text::REQUEST_RATE_LIMIT_EXCEEDED,
                    ))
                });
            if let utoipa::openapi::RefOr::T(response_value) = response {
                let _previous_header = response_value.headers.insert(
                    str_constants::text::RETRY_AFTER.to_owned(),
                    utoipa::openapi::header::Header::default(),
                );
            }
        });
    if let Some(components) = document.components.as_mut() {
        components.add_security_scheme(
            str_constants::text::ADMIN_COOKIE,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        str_constants::server_admin::ACCESS_COOKIE_NAME,
                        str_constants::text::HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE,
                    ),
                ),
            ),
        );
        components.add_security_scheme(
            str_constants::text::ADMIN_CSRF,
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        str_constants::text::X_CSRF_TOKEN,
                        str_constants::text::CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION,
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
                str_constants::admin_api_paths::AUTH_SIGN_IN,
                axum::routing::post(super::sign_in),
            )
            .route(
                str_constants::admin_api_paths::AUTH_REFRESH,
                axum::routing::post(super::refresh),
            )
            .route(
                str_constants::admin_api_paths::AUTH_SIGN_OUT,
                axum::routing::post(super::sign_out),
            )
            .route(
                str_constants::admin_api_paths::AUTH_ME,
                axum::routing::get(super::me),
            )
            .route(
                str_constants::admin_api_paths::AUTH_SESSIONS,
                axum::routing::get(super::sessions).delete(super::revoke_all_sessions),
            )
            .route(
                str_constants::admin_api_paths::AUTH_SESSION,
                axum::routing::delete(super::revoke_session),
            )
            .route(
                str_constants::admin_api_paths::USERS,
                axum::routing::get(super::list_users).post(super::create_user),
            )
            .route(
                str_constants::admin_api_paths::USER,
                axum::routing::patch(super::update_user).delete(super::delete_user),
            )
            .route(
                str_constants::admin_api_paths::USER_PASSWORD,
                axum::routing::post(super::set_user_password),
            )
            .route(
                str_constants::admin_api_paths::USER_BAN,
                axum::routing::post(super::set_user_ban),
            )
            .route(
                str_constants::admin_api_paths::ROLES,
                axum::routing::get(super::list_roles).post(super::create_role),
            )
            .route(
                str_constants::admin_api_paths::ROLE,
                axum::routing::patch(super::update_role).delete(super::delete_role),
            )
            .route(
                str_constants::admin_api_paths::ROLE_PERMISSIONS,
                axum::routing::put(super::set_role_permissions),
            )
            .route(
                str_constants::admin_api_paths::USER_ROLES,
                axum::routing::put(super::set_user_roles),
            )
            .route(
                str_constants::admin_api_paths::PERMISSIONS,
                axum::routing::get(super::list_permissions),
            )
            .route(
                str_constants::admin_api_paths::AUDIT,
                axum::routing::get(super::audit_log),
            )
            .route(
                str_constants::admin_api_paths::SETTINGS,
                axum::routing::get(super::settings).patch(super::update_settings),
            )
            .method_not_allowed_fallback(async || super::AdminApiError::MethodNotAllowed)
            .layer(axum::extract::DefaultBodyLimit::max(65_536usize))
            .with_state(state),
    )
}
