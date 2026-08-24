#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
struct ClientTransport;
impl frontend_contract::Transport for ClientTransport {
    fn send(
        &self,
        _request: frontend_contract::TransportRequest,
    ) -> impl Future<
        Output = Result<frontend_contract::TransportResponse, frontend_contract::TransportError>,
    > + '_ {
        std::future::ready(Err(frontend_contract::TransportError::default()))
    }
}
#[test]
#[allow(clippy::needless_for_each)] // iterator assertions follow the workspace no-for-loop policy
fn every_admin_api_route_has_named_route_and_client_functions() {
    assert_eq!(
        <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::ROUTE_COUNT,
        28usize
    );
    assert_eq!(
        super::metrics_route(),
        super::AdminRoute::Metrics.contract().path()
    );
    assert_eq!(
        super::open_api_route(),
        super::AdminRoute::OpenApi.contract().path()
    );
    assert_eq!(
        super::version_route(),
        super::AdminRoute::Version.contract().path()
    );
    [
        size_of_val(&super::audit_log_route),
        size_of_val(&super::export_audit_log_route),
        size_of_val(&super::branding_route),
        size_of_val(&super::read_data_table_route),
        size_of_val(&super::list_data_tables_route),
        size_of_val(&super::change_own_password_route),
        size_of_val(&super::create_role_route),
        size_of_val(&super::create_user_route),
        size_of_val(&super::delete_role_route),
        size_of_val(&super::delete_user_route),
        size_of_val(&super::me_route),
        size_of_val(&super::metrics_route),
        size_of_val(&super::open_api_route),
        size_of_val(&super::list_permissions_route),
        size_of_val(&super::refresh_route),
        size_of_val(&super::revoke_all_sessions_route),
        size_of_val(&super::revoke_session_route),
        size_of_val(&super::list_roles_route),
        size_of_val(&super::set_role_permissions_route),
        size_of_val(&super::set_user_ban_route),
        size_of_val(&super::set_user_password_route),
        size_of_val(&super::set_user_roles_route),
        size_of_val(&super::settings_route),
        size_of_val(&super::sign_in_route),
        size_of_val(&super::sign_out_route),
        size_of_val(&super::sessions_route),
        size_of_val(&super::update_role_route),
        size_of_val(&super::update_settings_route),
        size_of_val(&super::update_user_route),
        size_of_val(&super::list_users_route),
        size_of_val(&super::version_route),
    ]
    .into_iter()
    .for_each(|size| assert_eq!(size, usize_constants::ZERO));
    [
        size_of_val(&super::audit_log_client::<ClientTransport>),
        size_of_val(&super::export_audit_log_client::<ClientTransport>),
        size_of_val(&super::branding_client::<ClientTransport>),
        size_of_val(&super::read_data_table_client::<ClientTransport>),
        size_of_val(&super::list_data_tables_client::<ClientTransport>),
        size_of_val(&super::change_own_password_client::<ClientTransport>),
        size_of_val(&super::create_role_client::<ClientTransport>),
        size_of_val(&super::create_user_client::<ClientTransport>),
        size_of_val(&super::delete_role_client::<ClientTransport>),
        size_of_val(&super::delete_user_client::<ClientTransport>),
        size_of_val(&super::me_client::<ClientTransport>),
        size_of_val(&super::metrics_client::<ClientTransport>),
        size_of_val(&super::open_api_client::<ClientTransport>),
        size_of_val(&super::list_permissions_client::<ClientTransport>),
        size_of_val(&super::refresh_client::<ClientTransport>),
        size_of_val(&super::revoke_all_sessions_client::<ClientTransport>),
        size_of_val(&super::revoke_session_client::<ClientTransport>),
        size_of_val(&super::list_roles_client::<ClientTransport>),
        size_of_val(&super::set_role_permissions_client::<ClientTransport>),
        size_of_val(&super::set_user_ban_client::<ClientTransport>),
        size_of_val(&super::set_user_password_client::<ClientTransport>),
        size_of_val(&super::set_user_roles_client::<ClientTransport>),
        size_of_val(&super::settings_client::<ClientTransport>),
        size_of_val(&super::sign_in_client::<ClientTransport>),
        size_of_val(&super::sign_out_client::<ClientTransport>),
        size_of_val(&super::sessions_client::<ClientTransport>),
        size_of_val(&super::update_role_client::<ClientTransport>),
        size_of_val(&super::update_settings_client::<ClientTransport>),
        size_of_val(&super::update_user_client::<ClientTransport>),
        size_of_val(&super::list_users_client::<ClientTransport>),
        size_of_val(&super::version_client::<ClientTransport>),
    ]
    .into_iter()
    .for_each(|size| assert_eq!(size, usize_constants::ZERO));
}
fn assert_rejects_unknown_field<Value>(json: &str)
where
    Value: serde::de::DeserializeOwned,
{
    let Err(_error) = serde_json::from_str::<Value>(json) else {
        panic!("30bbf690");
    };
}
#[test]
fn administrator_collections_enforce_item_limit_for_construction_and_deserialization() {
    let maximum_values = vec![
        super::AdminRoleId::try_from(i64_constants::ONE).expect("4cd8c4ef administrator_collections_enforce_item_limit_for_construction_and_deserialization invariant must hold");
        super::ADMIN_COLLECTION_MAX_ITEMS
    ];
    let Ok(maximum_role_ids) = super::AdminRoleIds::try_from(maximum_values) else {
        panic!("bce86c7b");
    };
    assert_eq!(
        maximum_role_ids.as_ref().len(),
        super::ADMIN_COLLECTION_MAX_ITEMS
    );
    let oversized = vec![
        super::AdminRoleId::try_from(i64_constants::ONE).expect("1c1b920f administrator_collections_enforce_item_limit_for_construction_and_deserialization invariant must hold");
        super::ADMIN_COLLECTION_MAX_ITEMS.saturating_add(usize_constants::ONE)
    ];
    assert!(matches!(
        super::AdminRoleIds::try_from(oversized),
        Err(super::AdminCollectionError::TooLong)
    ));
    let json = serde_json::json!(vec![
        i64_constants::ONE;
        super::ADMIN_COLLECTION_MAX_ITEMS
            .saturating_add(usize_constants::ONE)
    ])
    .to_string();
    let Err(_error) = serde_json::from_str::<super::AdminRoleIds>(&json) else {
        panic!("742a0bdd");
    };
}
#[test]
fn authentication_route_family_has_valid_coverage() {
    let descriptors = <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors();
    assert_eq!(descriptors.as_ref().len(), 28usize);
    assert_eq!(
        frontend_contract::validate_route_coverage(descriptors.as_ref()),
        Ok(())
    );
    assert_eq!(
        <super::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
            .map(frontend_contract::RouteBodyLimit::get),
        Some(super::admin_api_body_max_bytes().get())
    );
}
#[test]
fn request_payloads_reject_unknown_fields() {
    assert_rejects_unknown_field::<super::AdminSignInReq>(
        str_constants::LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminCreateUserReq>(
        str_constants::DISPLAY_NAME_ADMIN_LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminUpdateUserReq>(
        str_constants::DISPLAY_NAME_ADMIN_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminSetUserPasswordReq>(
        str_constants::PASSWORD_SECRET_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminSetUserBanReq>(
        str_constants::IS_BANNED_TRUE_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminCreateRoleReq>(
        str_constants::NAME_ADMINISTRATOR_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminUpdateRoleReq>(
        str_constants::NAME_ADMINISTRATOR_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminSetUserRolesReq>(
        str_constants::ROLE_IDS_1_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminSetRolePermissionsReq>(
        str_constants::PERMISSION_IDS_1_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<super::AdminUpdateSettingsReq>(
        str_constants::SITE_NAME_ADMIN_UNKNOWN_TRUE,
    );
}
#[test]
fn route_contract_keeps_custom_action_policy_and_path_together() {
    let route = super::AdminRoute::SetUserBan(super::AdminUserId::try_from(7).expect(
        "8bed843c route_contract_keeps_custom_action_policy_and_path_together invariant must hold",
    ));
    assert_eq!(route.path().as_ref(), "/v1/admin/users/7/ban");
    assert_eq!(
        route.contract().method(),
        frontend_contract::HttpMethod::Post
    );
    assert_eq!(
        route.contract().mutation(),
        frontend_contract::MutationKind::Mutating
    );
    assert_eq!(
        route.contract().authentication(),
        frontend_contract::AuthenticationRequirement::Permission(
            frontend_contract::ContractStr::from(
                super::AdminPermission::UsersUpdate.as_str().get(),
            )
        )
    );
}
#[test]
fn parameterized_admin_route_path_uses_typed_route_metadata() {
    let session_id = super::AdminSessionIdentifier::try_from(String::from("test-session")).expect(
        "84d51132 parameterized_admin_route_path_uses_typed_route_metadata invariant must hold",
    );
    let role_id = super::AdminRoleId::try_from(7i64).expect(
        "1d69f24c parameterized_admin_route_path_uses_typed_route_metadata invariant must hold",
    );
    let user_id = super::AdminUserId::try_from(8i64).expect(
        "35959579 parameterized_admin_route_path_uses_typed_route_metadata invariant must hold",
    );
    let path = super::admin_parameterized_route_path::<super::AdminRevokeSessionRoute>(&session_id);
    assert_eq!(path.as_ref(), "/v1/admin/auth/sessions/test-session");
    assert_eq!(
        String::from(super::read_data_table_route(&super::AdminDataTable::Roles)),
        "/tables/roles"
    );
    assert_eq!(String::from(super::delete_role_route(&role_id)), "/roles/7");
    assert_eq!(String::from(super::delete_user_route(&user_id)), "/users/8");
    assert_eq!(
        String::from(super::revoke_session_route(&session_id)),
        "/auth/sessions/test-session"
    );
    assert_eq!(
        String::from(super::set_role_permissions_route(&role_id)),
        "/roles/7/permissions"
    );
    assert_eq!(
        String::from(super::set_user_ban_route(&user_id)),
        "/users/8/ban"
    );
    assert_eq!(
        String::from(super::set_user_password_route(&user_id)),
        "/users/8/password"
    );
    assert_eq!(
        String::from(super::set_user_roles_route(&user_id)),
        "/users/8/roles"
    );
    assert_eq!(String::from(super::update_role_route(&role_id)), "/roles/7");
    assert_eq!(String::from(super::update_user_route(&user_id)), "/users/8");
}
#[test]
fn html_action_inventory_has_unique_paths() {
    let paths = super::AdminHtmlAction::ALL
        .into_iter()
        .map(super::AdminHtmlAction::get)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(paths.len(), super::AdminHtmlAction::ALL.len());
}
#[test]
fn open_api_page_uses_the_typed_authenticated_api_route() {
    let route = super::AdminRoute::OpenApi;
    assert_eq!(route.path().as_ref(), "/v1/admin/openapi.json");
    assert_eq!(
        route.contract().authentication(),
        frontend_contract::AuthenticationRequirement::Permission(
            frontend_contract::ContractStr::from(
                super::AdminPermission::OpenApiRead.as_str().get(),
            ),
        )
    );
    assert_eq!(
        super::AdminPage::OpenApi.route(),
        Some(super::AdminRoute::OpenApi)
    );
    assert_eq!(
        super::AdminPage::OpenApi.spec().capability(),
        super::AdminPageCapability::Swagger
    );
    assert!(super::AdminPage::all().all(|page| {
        page == super::AdminPage::OpenApi
            || page.spec().capability() == super::AdminPageCapability::Always
    }));
}
#[test]
fn removed_audit_log_page_is_not_a_frontend_route() {
    assert_eq!(
        super::AdminPage::from_path(super::AdminPagePathRef::from("/admin/audit-log")),
        None
    );
    let Err(_error) = super::AdminDefaultRoute::try_from(String::from("/admin/audit-log")) else {
        panic!("61f0ab3e");
    };
}
#[test]
fn administrator_routes_use_snake_case_segments() {
    let frontend_paths = super::AdminFrontendPath::all_pages()
        .map(super::AdminFrontendPath::get)
        .collect::<Vec<_>>();
    assert_eq!(
        frontend_paths.len(),
        super::AdminPage::specs().len().saturating_add(2usize)
    );
    assert_eq!(
        frontend_paths
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        frontend_paths.len()
    );
    assert!(
        [
            super::AdminRoute::Audit,
            super::AdminRoute::AuditExport,
            super::AdminRoute::Settings,
            super::AdminRoute::SignIn,
            super::AdminRoute::SignOut,
        ]
        .iter()
        .all(|route| !route.path().as_ref().contains('-'))
    );
    assert!(frontend_paths.iter().all(|path| !path.contains('-')));
    assert!(
        super::AdminHtmlAction::ALL
            .iter()
            .all(|action| !action.get().contains('-'))
    );
}

#[test]
fn administrator_crud_frontend_paths_are_dedicated_pages() {
    assert_eq!(
        super::AdminFrontendPath::UsersCreate.get(),
        "/admin/users/create"
    );
    assert_eq!(
        super::AdminFrontendPath::UsersManage.get(),
        "/admin/users/manage"
    );
    assert_eq!(
        super::AdminFrontendPath::RolesCreate.get(),
        "/admin/roles/create"
    );
    assert_eq!(
        super::AdminFrontendPath::RolesManage.get(),
        "/admin/roles/manage"
    );
}
#[test]
fn audit_details_enforce_serialized_byte_limit() {
    let accepted = super::SerdeJsonAdminAuditDetails::try_from(serde_json::json!({
        "operation": "create"
    }));
    let _accepted =
        accepted.expect("20697dc1 audit_details_enforce_serialized_byte_limit invariant must hold");
    let oversized = super::SerdeJsonAdminAuditDetails::try_from(serde_json::Value::String(
        str_constants::A_ALT.repeat(super::ADMIN_AUDIT_DETAILS_MAX_BYTES),
    ));
    assert_eq!(
        oversized.err(),
        Some(super::AdminAuditDetailsTooLarge(
            super::AdminAuditDetailsBytes::from(
                super::ADMIN_AUDIT_DETAILS_MAX_BYTES.saturating_add(2usize),
            ),
        ))
    );
}
#[test]
fn table_sort_fields_reject_unknown_and_wrong_table_keys() {
    assert_eq!(
        super::AdminTableSortField::try_from_key(
            &super::AdminTableSortField::USER,
            super::AdminTableSortKeyRef::from(str_constants::LOGIN),
        ),
        Ok(super::AdminTableSortField::UserLogin)
    );
    assert_eq!(
        super::AdminTableSortField::try_from_key(
            &super::AdminTableSortField::USER,
            super::AdminTableSortKeyRef::from(str_constants::CREATED_AT),
        ),
        Err(super::AdminTableSortFieldTryFromKeyError)
    );
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods"
)]
fn data_tables_round_trip_and_require_read_permissions() {
    assert_eq!(super::AdminDataTable::ALL.len(), 12usize);
    assert_eq!(
        super::AdminDataTable::PG_ORDER,
        [
            super::AdminDataTable::Users,
            super::AdminDataTable::Roles,
            super::AdminDataTable::Permissions,
            super::AdminDataTable::UserRoles,
            super::AdminDataTable::RolePermissions,
            super::AdminDataTable::RefreshTokens,
            super::AdminDataTable::AccessSessions,
            super::AdminDataTable::LoginAttempts,
            super::AdminDataTable::AuditLog,
            super::AdminDataTable::SystemSettings,
            super::AdminDataTable::RateLimits,
            super::AdminDataTable::CleanupStatus,
        ]
    );
    assert_eq!(
        super::AdminPage::navigation().collect::<Vec<_>>(),
        vec![
            super::AdminPage::OpenApi,
            super::AdminPage::Metrics,
            super::AdminPage::Profile,
            super::AdminPage::Sessions,
            super::AdminPage::Settings,
            super::AdminPage::Version,
        ]
    );
    assert_eq!(
        super::AdminPage::all()
            .filter(|page| bool::from(page.supports_csr()))
            .collect::<Vec<_>>(),
        vec![
            super::AdminPage::Users,
            super::AdminPage::Roles,
            super::AdminPage::Permissions,
            super::AdminPage::Settings,
            super::AdminPage::Tables,
            super::AdminPage::Sessions,
            super::AdminPage::Profile,
        ]
    );
    assert_eq!(
        super::AdminPage::all()
            .filter(|page| bool::from(page.uses_table_query()))
            .collect::<Vec<_>>(),
        vec![
            super::AdminPage::Users,
            super::AdminPage::Roles,
            super::AdminPage::Permissions,
        ]
    );
    assert_eq!(
        super::AdminPage::navigation()
            .map(|page| page.spec().route_name().to_string())
            .collect::<Vec<_>>(),
        vec![
            String::from("swagger_ui"),
            String::from("metrics"),
            String::from("profile"),
            String::from("sessions"),
            String::from("settings"),
            String::from("version"),
        ]
    );
    assert_eq!(
        super::AdminHtmlAction::SignOut.route_name().as_ref(),
        "sign_out"
    );
    assert_eq!(
        frontend_contract::HandlerContract::method(super::AdminHtmlAction::SignOut),
        frontend_contract::RouteMethod::Post
    );
    assert_eq!(
        frontend_contract::HandlerContract::path(super::AdminHtmlAction::SignOut).get(),
        super::AdminHtmlAction::SignOut.get()
    );
    assert_eq!(
        frontend_contract::HandlerContract::method(super::AdminFrontendPath::Settings),
        frontend_contract::RouteMethod::Get
    );
    assert_eq!(
        frontend_contract::HandlerContract::path(super::AdminFrontendPath::Settings).get(),
        super::AdminFrontendPath::Settings.get()
    );
    assert!(super::AdminPage::navigation().all(|page| {
        let page_label = page.spec().route_name();
        super::AdminDataTable::PG_ORDER
            .iter()
            .all(|table| table.to_string() != page_label.as_ref())
    }));
    assert_eq!(
        super::AdminDataTable::ALL
            .into_iter()
            .filter(|table| bool::from(table.supports_filters()))
            .collect::<Vec<_>>(),
        vec![super::AdminDataTable::RolePermissions]
    );
    assert_eq!(
        super::AdminDataTable::PG_ORDER.map(|table| table.frontend_path().to_string()),
        [
            String::from("/admin/users"),
            String::from("/admin/roles"),
            String::from("/admin/permissions"),
            String::from("/admin/user_roles"),
            String::from("/admin/role_permissions"),
            String::from("/admin/refresh_tokens"),
            String::from("/admin/access_sessions"),
            String::from("/admin/login_attempts"),
            String::from("/admin/audit_log"),
            String::from("/admin/system_settings"),
            String::from("/admin/rate_limits"),
            String::from("/admin/cleanup_status"),
        ]
    );
    super::AdminDataTable::ALL.into_iter().for_each(|table| {
        let spec = table.spec();
        let columns = spec.columns().get().split(',').collect::<Vec<_>>();
        assert!(!columns.is_empty());
        assert_eq!(
            columns
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            columns.len()
        );
        assert!(!spec.order().get().is_empty());
        assert_eq!(spec.permission(), table.permission());
        assert_eq!(spec.supports_filters(), table.supports_filters());
        assert_eq!(
            super::AdminDataTable::try_from(table.to_string()).expect(
                "0596134b data_tables_round_trip_and_require_read_permissions invariant must hold"
            ),
            table
        );
        assert_eq!(
            super::AdminDataTable::from_frontend_path(super::AdminPagePathRef::from(
                table.frontend_path().as_ref(),
            )),
            Some(table)
        );
        assert!(table.permission().as_str().get().ends_with(":read"));
    });
    assert_eq!(
        super::AdminDataTable::from_frontend_path(super::AdminPagePathRef::from("/admin/tables",)),
        None
    );
    assert_eq!(
        super::AdminDataTable::from_frontend_path(super::AdminPagePathRef::from("/admin/profile",)),
        None
    );
}

#[test]
fn administrator_identifiers_require_positive_database_values() {
    let _user_error = super::AdminUserId::try_from(i64_constants::ZERO).expect_err("6088ff6a");
    let _role_error = super::AdminRoleId::try_from(-i64_constants::ONE).expect_err("4406ffcc");
    let _permission_error =
        super::AdminPermissionId::try_from(i64_constants::ZERO).expect_err("f5d79bb8");
    let _audit_error = super::AdminAuditLogId::try_from(-i64_constants::ONE).expect_err("3ca5fe6c");
}
