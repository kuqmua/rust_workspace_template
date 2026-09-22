#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_frontend_contract_derive_route_catalog::RouteCatalog,
)]
#[route_catalog(
    family = AdminAuthenticationRouteFamily,
    body_limit = crate::admin_api_body_max_bytes::ADMIN_API_BODY_MAX_BYTES_VALUE,
)]
pub enum AdminRoute {
    #[route_catalog_route(
        crate::admin_delete_access_sessions_route::AdminDeleteAccessSessionsRoute
    )]
    DeleteAccessSessions,
    #[route_catalog_route(crate::admin_cleanup_status_table_route::AdminCleanupStatusTableRoute)]
    CleanupStatusTable,
    #[route_catalog_route(crate::admin_rate_limits_table_route::AdminRateLimitsTableRoute)]
    RateLimitsTable,
    #[route_catalog_route(crate::admin_login_attempts_table_route::AdminLoginAttemptsTableRoute)]
    LoginAttemptsTable,
    #[route_catalog_route(
        contract = <crate::admin_access_sessions_table_route::AdminAccessSessionsTableRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_access_sessions_table_route::AdminAccessSessionsTableRoute>(),
        exclude_from_family,
    )]
    AccessSessionsTable,
    #[route_catalog_route(crate::admin_refresh_tokens_table_route::AdminRefreshTokensTableRoute)]
    RefreshTokensTable,
    #[route_catalog_route(
        contract = <crate::admin_role_rules_table_route::AdminRoleRulesTableRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_role_rules_table_route::AdminRoleRulesTableRoute>(),
        exclude_from_family,
    )]
    RoleRulesTable,
    #[route_catalog_route(crate::admin_user_roles_table_route::AdminUserRolesTableRoute)]
    UserRolesTable,
    #[route_catalog_route(
        contract = <crate::admin_read_audit_log_route::AdminReadAuditLogRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_read_audit_log_route::AdminReadAuditLogRoute>(),
        exclude_from_family,
    )]
    AuditLog,
    #[route_catalog_route(
        contract = <crate::admin_read_system_settings_route::AdminReadSystemSettingsRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_read_system_settings_route::AdminReadSystemSettingsRoute>(),
        exclude_from_family,
    )]
    SystemSettings,
    #[route_catalog_route(crate::admin_branding_route::AdminBrandingRoute)]
    Branding,
    #[route_catalog_route(crate::admin_data_tables_route::AdminDataTablesRoute)]
    DataTables,
    #[route_catalog_route(crate::admin_change_own_password_route::AdminChangeOwnPasswordRoute)]
    ChangeOwnPassword,
    #[route_catalog_route(crate::admin_create_roles_route::AdminCreateRolesRoute)]
    CreateRoles,
    #[route_catalog_route(
        crate::admin_create_roles_payload_example_route::AdminCreateRolesPayloadExampleRoute
    )]
    CreateRolesPayloadExample,
    #[route_catalog_route(
        crate::admin_create_user_payload_example_route::AdminCreateUserPayloadExampleRoute
    )]
    CreateUserPayloadExample,
    #[route_catalog_route(crate::admin_create_user_route::AdminCreateUserRoute)]
    CreateUser,
    #[route_catalog_route(crate::admin_delete_roles_route::AdminDeleteRolesRoute)]
    DeleteRoles,
    #[route_catalog_route(
        crate::admin_delete_roles_payload_example_route::AdminDeleteRolesPayloadExampleRoute
    )]
    DeleteRolesPayloadExample,
    #[route_catalog_route(crate::admin_delete_users_route::AdminDeleteUsersRoute)]
    DeleteUsers,
    #[route_catalog_route(
        crate::admin_delete_users_payload_example_route::AdminDeleteUsersPayloadExampleRoute
    )]
    DeleteUsersPayloadExample,
    #[route_catalog_route(crate::admin_me_route::AdminMeRoute)]
    Me,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            crate::admin_rule_requirement::admin_rule_requirement(crate::admin_rule::AdminRule::MetricsRead),
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from(constants_str::METRICS_READ),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = constants_str::METRICS_READ,
        exclude_from_family,
    )]
    Metrics,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            crate::admin_rule_requirement::admin_rule_requirement(crate::admin_rule::AdminRule::OpenApiRead),
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from(constants_str::OPENAPI_JSON_READ),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = constants_str::OPENAPI_JSON_READ,
        exclude_from_family,
    )]
    OpenApi,
    #[route_catalog_route(
        contract = <crate::admin_read_rules_route::AdminReadRulesRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_read_rules_route::AdminReadRulesRoute>(),
        exclude_from_family,
    )]
    Rules,
    #[route_catalog_route(crate::admin_refresh_route::AdminRefreshRoute)]
    Refresh,
    #[route_catalog_route(crate::admin_revoke_all_sessions_route::AdminRevokeAllSessionsRoute)]
    RevokeAllSessions,
    #[route_catalog_route(crate::admin_revoke_session_route::AdminRevokeSessionRoute)]
    RevokeSession,
    #[route_catalog_route(
        contract = <crate::admin_read_roles_route::AdminReadRolesRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_read_roles_route::AdminReadRolesRoute>(),
        exclude_from_family,
    )]
    Roles,
    #[route_catalog_route(crate::admin_settings_route::AdminSettingsRoute)]
    Settings,
    #[route_catalog_route(crate::admin_sign_in_route::AdminSignInRoute)]
    SignIn,
    #[route_catalog_route(crate::admin_sign_out_route::AdminSignOutRoute)]
    SignOut,
    #[route_catalog_route(crate::admin_sessions_route::AdminSessionsRoute)]
    Sessions,
    #[route_catalog_route(crate::admin_update_roles_route::AdminUpdateRolesRoute)]
    UpdateRoles,
    #[route_catalog_route(
        crate::admin_update_roles_payload_example_route::AdminUpdateRolesPayloadExampleRoute
    )]
    UpdateRolesPayloadExample,
    #[route_catalog_route(crate::admin_update_settings_route::AdminUpdateSettingsRoute)]
    UpdateSettings,
    #[route_catalog_route(crate::admin_update_users_route::AdminUpdateUsersRoute)]
    UpdateUsers,
    #[route_catalog_route(
        crate::admin_update_users_payload_example_route::AdminUpdateUsersPayloadExampleRoute
    )]
    UpdateUsersPayloadExample,
    #[route_catalog_route(
        contract = <crate::admin_read_users_route::AdminReadUsersRoute as frontend_contract::typed_route::TypedRoute>::metadata().contract(),
        path = frontend_contract::typed_route_path::typed_route_path::<crate::admin_read_users_route::AdminReadUsersRoute>(),
        exclude_from_family,
    )]
    Users,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from("/health/read"),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = "/health/read",
        exclude_from_family,
    )]
    Health,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from("/health_check/read"),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = "/health_check/read",
        exclude_from_family,
    )]
    HealthCheck,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from("/health/live/read"),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = "/health/live/read",
        exclude_from_family,
    )]
    HealthLive,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from("/health/ready/read"),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = "/health/ready/read",
        exclude_from_family,
    )]
    HealthReady,
    #[route_catalog_route(
        contract = frontend_contract::route_contract::RouteContract::new(
            frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
            frontend_contract::route_method::RouteMethod::Get,
            frontend_contract::mutation_kind::MutationKind::ReadOnly,
            frontend_contract::contract_str::ContractStr::from(constants_str::COMMON_ROUTES_GIT_INFO_READ),
            frontend_contract::success_status::SuccessStatus::Code200,
        ),
        path = constants_str::COMMON_ROUTES_GIT_INFO_READ,
        exclude_from_family,
    )]
    Version,
}
impl AdminRoute {
    #[must_use]
    pub fn path(self) -> crate::admin_route_path::AdminRoutePath {
        let suffix = self.catalog_path();
        if matches!(
            self,
            Self::Version | Self::Health | Self::HealthCheck | Self::HealthLive | Self::HealthReady
        ) {
            crate::admin_route_path::AdminRoutePath::try_from(String::from(suffix))
                .unwrap_or_default()
        } else {
            crate::admin_api_route_path::admin_api_route_path(suffix)
        }
    }
}
impl frontend_contract::route_registration_contract::RouteRegistrationContract for AdminRoute {
    fn registration_method(self) -> frontend_contract::route_method::RouteMethod {
        self.contract().method()
    }

    fn registration_path(self) -> frontend_contract::contract_str::ContractStr {
        self.contract().path()
    }
}
