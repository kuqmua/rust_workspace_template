#![allow(
    unused_variables,
    reason = "test trait fixtures preserve repository type-based parameter names"
)]

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
struct ClientTransport;
impl frontend_contract::transport::Transport for ClientTransport {
    fn send(
        &self,
        transport_request: frontend_contract::transport_request::TransportRequest,
    ) -> impl Future<
        Output = Result<
            frontend_contract::transport_response::TransportResponse,
            frontend_contract::transport_error::TransportError,
        >,
    > + '_ {
        std::future::ready(Err(
            frontend_contract::transport_error::TransportError::default(),
        ))
    }
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "test tests domain types uses iterator traversal to comply with the workspace no-for-loop policy"
)]
fn test_every_admin_api_route_has_named_route_and_client_functions() {
    assert_eq!(
        <crate::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::ROUTE_COUNT,
        28usize
    );
    assert_eq!(
        crate::admin_route::metrics_route(),
        crate::admin_route::AdminRoute::Metrics.contract().path()
    );
    assert_eq!(
        crate::admin_route::open_api_route(),
        crate::admin_route::AdminRoute::OpenApi.contract().path()
    );
    assert_eq!(
        crate::admin_route::version_route(),
        crate::admin_route::AdminRoute::Version.contract().path()
    );
    [
        size_of_val(&crate::admin_audit_log_route::audit_log_route),
        size_of_val(&crate::admin_audit_export_route::export_audit_log_route),
        size_of_val(&crate::admin_branding_route::branding_route),
        size_of_val(&crate::admin_data_table_route::read_data_table_route),
        size_of_val(&crate::admin_data_tables_route::list_data_tables_route),
        size_of_val(&crate::admin_change_own_password_route::change_own_password_route),
        size_of_val(&crate::admin_create_role_route::create_role_route),
        size_of_val(&crate::admin_create_user_route::create_user_route),
        size_of_val(&crate::admin_delete_role_route::delete_role_route),
        size_of_val(&crate::admin_delete_user_route::delete_user_route),
        size_of_val(&crate::admin_me_route::me_route),
        size_of_val(&crate::admin_route::metrics_route),
        size_of_val(&crate::admin_route::open_api_route),
        size_of_val(&crate::admin_list_permissions_route::list_permissions_route),
        size_of_val(&crate::admin_refresh_route::refresh_route),
        size_of_val(&crate::admin_revoke_all_sessions_route::revoke_all_sessions_route),
        size_of_val(&crate::admin_revoke_session_route::revoke_session_route),
        size_of_val(&crate::admin_list_roles_route::list_roles_route),
        size_of_val(&crate::admin_set_role_permissions_route::set_role_permissions_route),
        size_of_val(&crate::admin_set_user_ban_route::set_user_ban_route),
        size_of_val(&crate::admin_set_user_password_route::set_user_password_route),
        size_of_val(&crate::admin_set_user_roles_route::set_user_roles_route),
        size_of_val(&crate::admin_settings_route::settings_route),
        size_of_val(&crate::admin_sign_in_route::sign_in_route),
        size_of_val(&crate::admin_sign_out_route::sign_out_route),
        size_of_val(&crate::admin_sessions_route::sessions_route),
        size_of_val(&crate::admin_update_role_route::update_role_route),
        size_of_val(&crate::admin_update_settings_route::update_settings_route),
        size_of_val(&crate::admin_update_user_route::update_user_route),
        size_of_val(&crate::admin_list_users_route::list_users_route),
        size_of_val(&crate::admin_route::version_route),
    ]
    .into_iter()
    .for_each(|size| assert_eq!(size, constants_usize::ZERO));
    [
        size_of_val(&crate::admin_audit_log_route::audit_log_client::<ClientTransport>),
        size_of_val(&crate::admin_audit_export_route::export_audit_log_client::<ClientTransport>),
        size_of_val(&crate::admin_branding_route::branding_client::<ClientTransport>),
        size_of_val(&crate::admin_data_table_route::read_data_table_client::<ClientTransport>),
        size_of_val(&crate::admin_data_tables_route::list_data_tables_client::<ClientTransport>),
        size_of_val(&crate::admin_change_own_password_route::change_own_password_client::<ClientTransport>),
        size_of_val(&crate::admin_create_role_route::create_role_client::<ClientTransport>),
        size_of_val(&crate::admin_create_user_route::create_user_client::<ClientTransport>),
        size_of_val(&crate::admin_delete_role_route::delete_role_client::<ClientTransport>),
        size_of_val(&crate::admin_delete_user_route::delete_user_client::<ClientTransport>),
        size_of_val(&crate::admin_me_route::me_client::<ClientTransport>),
        size_of_val(&crate::admin_route::metrics_client::<ClientTransport>),
        size_of_val(&crate::admin_route::open_api_client::<ClientTransport>),
        size_of_val(&crate::admin_list_permissions_route::list_permissions_client::<ClientTransport>),
        size_of_val(&crate::admin_refresh_route::refresh_client::<ClientTransport>),
        size_of_val(&crate::admin_revoke_all_sessions_route::revoke_all_sessions_client::<ClientTransport>),
        size_of_val(&crate::admin_revoke_session_route::revoke_session_client::<ClientTransport>),
        size_of_val(&crate::admin_list_roles_route::list_roles_client::<ClientTransport>),
        size_of_val(&crate::admin_set_role_permissions_route::set_role_permissions_client::<ClientTransport>),
        size_of_val(&crate::admin_set_user_ban_route::set_user_ban_client::<ClientTransport>),
        size_of_val(&crate::admin_set_user_password_route::set_user_password_client::<ClientTransport>),
        size_of_val(&crate::admin_set_user_roles_route::set_user_roles_client::<ClientTransport>),
        size_of_val(&crate::admin_settings_route::settings_client::<ClientTransport>),
        size_of_val(&crate::admin_sign_in_route::sign_in_client::<ClientTransport>),
        size_of_val(&crate::admin_sign_out_route::sign_out_client::<ClientTransport>),
        size_of_val(&crate::admin_sessions_route::sessions_client::<ClientTransport>),
        size_of_val(&crate::admin_update_role_route::update_role_client::<ClientTransport>),
        size_of_val(&crate::admin_update_settings_route::update_settings_client::<ClientTransport>),
        size_of_val(&crate::admin_update_user_route::update_user_client::<ClientTransport>),
        size_of_val(&crate::admin_list_users_route::list_users_client::<ClientTransport>),
        size_of_val(&crate::admin_route::version_client::<ClientTransport>),
    ]
    .into_iter()
    .for_each(|size| assert_eq!(size, constants_usize::ZERO));
}
fn assert_rejects_unknown_field<Value>(str: &str)
where
    Value: serde::de::DeserializeOwned,
{
    let Err(_error) = serde_json::from_str::<Value>(str) else {
        std::panic::panic_any(constants_str::PANIC_30BBF690);
    };
}
#[test]
fn test_administrator_collections_enforce_item_limit_for_construction_and_deserialization() {
    let maximum_values = vec![
        crate::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
            .expect(constants_str::VALUE_E535AB72);
        crate::admin_collection_max_items::ADMIN_COLLECTION_MAX_ITEMS
    ];
    let Ok(maximum_role_ids) = crate::admin_role_ids::AdminRoleIds::try_from(maximum_values) else {
        std::panic::panic_any(constants_str::PANIC_BCE86C7B);
    };
    assert_eq!(
        maximum_role_ids.as_ref().len(),
        crate::admin_collection_max_items::ADMIN_COLLECTION_MAX_ITEMS
    );
    let oversized = vec![
        crate::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
            .expect(constants_str::VALUE_36ED0D08);
        crate::admin_collection_max_items::ADMIN_COLLECTION_MAX_ITEMS
            .saturating_add(constants_usize::ONE)
    ];
    assert!(matches!(
        crate::admin_role_ids::AdminRoleIds::try_from(oversized),
        Err(crate::admin_collection_error::AdminCollectionError::TooLong)
    ));
    let oversized_json_values = vec![
        constants_i64::ONE;
        crate::admin_collection_max_items::ADMIN_COLLECTION_MAX_ITEMS
            .saturating_add(constants_usize::ONE)
    ];
    let json = serde_json::json!(oversized_json_values).to_string();
    let Err(_error) = serde_json::from_str::<crate::admin_role_ids::AdminRoleIds>(&json) else {
        std::panic::panic_any(constants_str::PANIC_742A0BDD);
    };
}
#[test]
fn test_authentication_route_family_has_valid_coverage() {
    let descriptors = <crate::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::coverage_descriptors();
    assert_eq!(descriptors.as_ref().len(), 28usize);
    assert_eq!(
        frontend_contract::validate_route_coverage::validate_route_coverage(descriptors.as_ref()),
        Ok(())
    );
    assert_eq!(
        <crate::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
            .map(frontend_contract::route_body_limit::RouteBodyLimit::get),
        Some(crate::default_admin_api_body_max_bytes::default_admin_api_body_max_bytes().get())
    );
}
#[test]
fn test_request_payloads_reject_unknown_fields() {
    assert_rejects_unknown_field::<crate::admin_sign_in_request::AdminSignInRequest>(
        constants_str::LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<crate::admin_create_user_request::AdminCreateUserRequest>(
        constants_str::DISPLAY_NAME_ADMIN_LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<crate::admin_update_user_request::AdminUpdateUserRequest>(
        constants_str::DISPLAY_NAME_ADMIN_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<
        crate::admin_set_user_password_request::AdminSetUserPasswordRequest,
    >(constants_str::PASSWORD_SECRET_UNKNOWN_TRUE);
    assert_rejects_unknown_field::<crate::admin_set_user_ban_request::AdminSetUserBanRequest>(
        constants_str::IS_BANNED_TRUE_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<crate::admin_create_role_request::AdminCreateRoleRequest>(
        constants_str::NAME_ADMINISTRATOR_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<crate::admin_update_role_request::AdminUpdateRoleRequest>(
        constants_str::NAME_ADMINISTRATOR_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<crate::admin_set_user_roles_request::AdminSetUserRolesRequest>(
        constants_str::ROLE_IDS_1_UNKNOWN_TRUE,
    );
    assert_rejects_unknown_field::<
        crate::admin_set_role_permissions_request::AdminSetRolePermissionsRequest,
    >(constants_str::PERMISSION_IDS_1_UNKNOWN_TRUE);
    assert_rejects_unknown_field::<crate::admin_update_settings_request::AdminUpdateSettingsRequest>(
        constants_str::SITE_NAME_ADMIN_UNKNOWN_TRUE,
    );
}
#[test]
fn test_route_contract_keeps_custom_action_policy_and_path_together() {
    let route = crate::admin_route::AdminRoute::SetUserBan(
        crate::admin_user_id::AdminUserId::try_from(7).expect(constants_str::DIAGNOSTIC_8BED843C),
    );
    assert_eq!(route.path().as_ref(), constants_str::VALUE_FC4871BE);
    assert_eq!(
        route.contract().method(),
        frontend_contract::route_method::RouteMethod::Post
    );
    assert_eq!(
        route.contract().mutation(),
        frontend_contract::mutation_kind::MutationKind::Mutating
    );
    assert_eq!(
        route.contract().authentication(),
        frontend_contract::authentication_requirement::AuthenticationRequirement::Permission(
            frontend_contract::contract_str::ContractStr::from(
                crate::admin_permission::AdminPermission::UsersUpdate
                    .as_str()
                    .get(),
            )
        )
    );
}
#[test]
fn test_parameterized_admin_route_path_uses_typed_route_metadata() {
    let session_id = crate::admin_session_identifier::AdminSessionIdentifier::try_from(
        String::from(constants_str::VALUE_4943E43B),
    )
    .expect(constants_str::DIAGNOSTIC_84D51132);
    let role_id = crate::admin_role_id::AdminRoleId::try_from(7i64)
        .expect(constants_str::DIAGNOSTIC_1D69F24C);
    let user_id = crate::admin_user_id::AdminUserId::try_from(8i64)
        .expect(constants_str::DIAGNOSTIC_35959579);
    let path = crate::admin_parameterized_route_path::admin_parameterized_route_path::<
        crate::admin_revoke_session_route::AdminRevokeSessionRoute,
    >(&session_id);
    assert_eq!(path.as_ref(), constants_str::VALUE_C0FE54AF);
    assert_eq!(
        String::from(crate::admin_data_table_route::read_data_table_route(
            &crate::admin_data_table::AdminDataTable::Roles
        )),
        constants_str::VALUE_BCEDACF8
    );
    assert_eq!(
        String::from(crate::admin_delete_role_route::delete_role_route(&role_id)),
        constants_str::VALUE_5DE652EF
    );
    assert_eq!(
        String::from(crate::admin_delete_user_route::delete_user_route(&user_id)),
        constants_str::VALUE_769BBFA3
    );
    assert_eq!(
        String::from(crate::admin_revoke_session_route::revoke_session_route(
            &session_id
        )),
        constants_str::VALUE_FEF7B989
    );
    assert_eq!(
        String::from(crate::admin_set_role_permissions_route::set_role_permissions_route(&role_id)),
        constants_str::VALUE_BF4EA24D
    );
    assert_eq!(
        String::from(crate::admin_set_user_ban_route::set_user_ban_route(
            &user_id
        )),
        constants_str::VALUE_761F4C43
    );
    assert_eq!(
        String::from(crate::admin_set_user_password_route::set_user_password_route(&user_id)),
        constants_str::VALUE_21E2A4C7
    );
    assert_eq!(
        String::from(crate::admin_set_user_roles_route::set_user_roles_route(
            &user_id
        )),
        constants_str::VALUE_EF3D9D78
    );
    assert_eq!(
        String::from(crate::admin_update_role_route::update_role_route(&role_id)),
        constants_str::VALUE_5DE652EF
    );
    assert_eq!(
        String::from(crate::admin_update_user_route::update_user_route(&user_id)),
        constants_str::VALUE_769BBFA3
    );
}
#[test]
fn test_html_action_inventory_has_unique_paths() {
    let paths = crate::admin_html_action::AdminHtmlAction::ALL
        .into_iter()
        .map(crate::admin_html_action::AdminHtmlAction::get)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        paths.len(),
        crate::admin_html_action::AdminHtmlAction::ALL.len()
    );
}
#[test]
fn test_open_api_page_uses_the_typed_authenticated_api_route() {
    let route = crate::admin_route::AdminRoute::OpenApi;
    assert_eq!(route.path().as_ref(), constants_str::VALUE_72ACA5B8);
    assert_eq!(
        route.contract().authentication(),
        frontend_contract::authentication_requirement::AuthenticationRequirement::Permission(
            frontend_contract::contract_str::ContractStr::from(
                crate::admin_permission::AdminPermission::OpenApiRead
                    .as_str()
                    .get(),
            ),
        )
    );
    assert_eq!(
        crate::admin_page::AdminPage::OpenApi.route(),
        Some(crate::admin_route::AdminRoute::OpenApi)
    );
    assert_eq!(
        crate::admin_page::AdminPage::OpenApi.spec().capability(),
        crate::admin_page_capability::AdminPageCapability::Swagger
    );
    assert!(crate::admin_page::AdminPage::all().all(|page| {
        page == crate::admin_page::AdminPage::OpenApi
            || page.spec().capability() == crate::admin_page_capability::AdminPageCapability::Always
    }));
}
#[test]
fn test_removed_audit_log_page_is_not_a_frontend_route() {
    assert_eq!(
        crate::admin_page::AdminPage::from_path(
            crate::admin_page_path_ref::AdminPagePathRef::from(constants_str::VALUE_FF160115)
        ),
        None
    );
    let Err(_error) = crate::admin_default_route::AdminDefaultRoute::try_from(String::from(
        constants_str::VALUE_FF160115,
    )) else {
        std::panic::panic_any(constants_str::PANIC_61F0AB3E);
    };
}
#[test]
fn test_administrator_routes_use_snake_case_segments() {
    let frontend_paths = crate::admin_frontend_path::AdminFrontendPath::all_pages()
        .map(crate::admin_frontend_path::AdminFrontendPath::get)
        .collect::<Vec<_>>();
    assert_eq!(
        frontend_paths.len(),
        crate::admin_page::AdminPage::specs()
            .len()
            .saturating_add(2usize)
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
            crate::admin_route::AdminRoute::Audit,
            crate::admin_route::AdminRoute::AuditExport,
            crate::admin_route::AdminRoute::Settings,
            crate::admin_route::AdminRoute::SignIn,
            crate::admin_route::AdminRoute::SignOut,
        ]
        .iter()
        .all(|route| !route.path().as_ref().contains('-'))
    );
    assert!(frontend_paths.iter().all(|path| !path.contains('-')));
    assert!(
        crate::admin_html_action::AdminHtmlAction::ALL
            .iter()
            .all(|action| !action.get().contains('-'))
    );
}

#[test]
fn test_administrator_crud_frontend_paths_are_dedicated_pages() {
    assert_eq!(
        crate::admin_frontend_path::AdminFrontendPath::UsersCreate.get(),
        constants_str::VALUE_846D4734
    );
    assert_eq!(
        crate::admin_frontend_path::AdminFrontendPath::UsersManage.get(),
        constants_str::VALUE_EE7B8FDF
    );
    assert_eq!(
        crate::admin_frontend_path::AdminFrontendPath::RolesCreate.get(),
        constants_str::VALUE_948D5A6C
    );
    assert_eq!(
        crate::admin_frontend_path::AdminFrontendPath::RolesManage.get(),
        constants_str::VALUE_49F831CD
    );
}
#[test]
fn test_audit_details_enforce_serialized_byte_limit() {
    let accepted = crate::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({
            "operation": "create"
        }),
    );
    let _accepted = accepted.expect(constants_str::DIAGNOSTIC_20697DC1);
    let oversized = crate::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
        serde_json::Value::String(
            constants_str::A_ALT
                .repeat(crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES),
        ),
    );
    assert_eq!(
        oversized.err(),
        Some(
            crate::admin_audit_details_too_large::AdminAuditDetailsTooLarge::from(
                crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(
                    crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES
                        .saturating_add(2usize),
                ),
            )
        )
    );
}
#[test]
fn test_table_sort_fields_reject_unknown_and_wrong_table_keys() {
    assert_eq!(
        crate::admin_table_sort_field::AdminTableSortField::try_from_key(
            &crate::admin_table_sort_field::AdminTableSortField::USER,
            crate::admin_table_sort_key_ref::AdminTableSortKeyRef::from(constants_str::LOGIN),
        ),
        Ok(crate::admin_table_sort_field::AdminTableSortField::UserLogin)
    );
    assert_eq!(
        crate::admin_table_sort_field::AdminTableSortField::try_from_key(
            &crate::admin_table_sort_field::AdminTableSortField::USER,
            crate::admin_table_sort_key_ref::AdminTableSortKeyRef::from(
                constants_str::CREATED_AT
            ),
        ),
        Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown)
    );
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods"
)]
fn test_data_tables_round_trip_and_require_read_permissions() {
    assert_eq!(crate::admin_data_table::AdminDataTable::ALL.len(), 12usize);
    assert_eq!(
        crate::admin_data_table::AdminDataTable::PG_ORDER,
        [
            crate::admin_data_table::AdminDataTable::Users,
            crate::admin_data_table::AdminDataTable::Roles,
            crate::admin_data_table::AdminDataTable::Permissions,
            crate::admin_data_table::AdminDataTable::UserRoles,
            crate::admin_data_table::AdminDataTable::RolePermissions,
            crate::admin_data_table::AdminDataTable::RefreshTokens,
            crate::admin_data_table::AdminDataTable::AccessSessions,
            crate::admin_data_table::AdminDataTable::LoginAttempts,
            crate::admin_data_table::AdminDataTable::AuditLog,
            crate::admin_data_table::AdminDataTable::SystemSettings,
            crate::admin_data_table::AdminDataTable::RateLimits,
            crate::admin_data_table::AdminDataTable::CleanupStatus,
        ]
    );
    assert_eq!(
        crate::admin_page::AdminPage::navigation().collect::<Vec<_>>(),
        [
            crate::admin_page::AdminPage::OpenApi,
            crate::admin_page::AdminPage::Metrics,
            crate::admin_page::AdminPage::Profile,
            crate::admin_page::AdminPage::Sessions,
            crate::admin_page::AdminPage::Settings,
            crate::admin_page::AdminPage::Version,
        ]
    );
    assert_eq!(
        crate::admin_page::AdminPage::all()
            .filter(|page| bool::from(page.supports_csr()))
            .collect::<Vec<_>>(),
        [
            crate::admin_page::AdminPage::Users,
            crate::admin_page::AdminPage::Roles,
            crate::admin_page::AdminPage::Permissions,
            crate::admin_page::AdminPage::Settings,
            crate::admin_page::AdminPage::Tables,
            crate::admin_page::AdminPage::Sessions,
            crate::admin_page::AdminPage::Profile,
        ]
    );
    assert_eq!(
        crate::admin_page::AdminPage::all()
            .filter(|page| bool::from(page.uses_table_query()))
            .collect::<Vec<_>>(),
        [
            crate::admin_page::AdminPage::Users,
            crate::admin_page::AdminPage::Roles,
            crate::admin_page::AdminPage::Permissions,
        ]
    );
    assert_eq!(
        crate::admin_page::AdminPage::navigation()
            .map(|page| page.spec().route_name().to_string())
            .collect::<Vec<_>>(),
        [
            String::from(constants_str::VALUE_E16D1963),
            String::from(constants_str::VALUE_177A7EA3),
            String::from(constants_str::VALUE_1900EAB6),
            String::from(constants_str::SESSIONS),
            String::from(constants_str::VALUE_CDE0FB0D),
            String::from(constants_str::VERSION_ALT_3),
        ]
    );
    assert_eq!(
        crate::admin_html_action::AdminHtmlAction::SignOut
            .route_name()
            .as_ref(),
        constants_str::SIGN_OUT
    );
    assert_eq!(
        frontend_contract::route_registration_contract::RouteRegistrationContract::method(
            crate::admin_html_action::AdminHtmlAction::SignOut
        ),
        frontend_contract::route_method::RouteMethod::Post
    );
    assert_eq!(
        frontend_contract::route_registration_contract::RouteRegistrationContract::path(
            crate::admin_html_action::AdminHtmlAction::SignOut
        )
        .get(),
        crate::admin_html_action::AdminHtmlAction::SignOut.get()
    );
    assert_eq!(
        frontend_contract::route_registration_contract::RouteRegistrationContract::method(
            crate::admin_frontend_path::AdminFrontendPath::Settings
        ),
        frontend_contract::route_method::RouteMethod::Get
    );
    assert_eq!(
        frontend_contract::route_registration_contract::RouteRegistrationContract::path(
            crate::admin_frontend_path::AdminFrontendPath::Settings
        )
        .get(),
        crate::admin_frontend_path::AdminFrontendPath::Settings.get()
    );
    assert!(crate::admin_page::AdminPage::navigation().all(|page| {
        let page_label = page.spec().route_name();
        crate::admin_data_table::AdminDataTable::PG_ORDER
            .iter()
            .all(|table| table.to_string() != page_label.as_ref())
    }));
    assert_eq!(
        crate::admin_data_table::AdminDataTable::ALL
            .into_iter()
            .filter(|table| bool::from(table.supports_filters()))
            .collect::<Vec<_>>(),
        [crate::admin_data_table::AdminDataTable::RolePermissions]
    );
    assert_eq!(
        crate::admin_data_table::AdminDataTable::PG_ORDER
            .map(|table| table.frontend_path().to_string()),
        [
            String::from(constants_str::VALUE_074B6E5E),
            String::from(constants_str::VALUE_DB2C56E6),
            String::from(constants_str::VALUE_84ECA72B),
            String::from(constants_str::VALUE_6B410750),
            String::from(constants_str::VALUE_1199CAE6),
            String::from(constants_str::VALUE_00257BE7),
            String::from(constants_str::VALUE_72DD61BC),
            String::from(constants_str::VALUE_1C0E5BAF),
            String::from(constants_str::VALUE_6F080C29),
            String::from(constants_str::VALUE_760FE0DF),
            String::from(constants_str::VALUE_20F6A07F),
            String::from(constants_str::VALUE_72852DE9),
        ]
    );
    crate::admin_data_table::AdminDataTable::ALL
        .into_iter()
        .for_each(|table| {
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
                crate::admin_data_table::AdminDataTable::try_from(table.to_string())
                    .expect(constants_str::DIAGNOSTIC_0596134B),
                table
            );
            assert_eq!(
                crate::admin_data_table::AdminDataTable::from_frontend_path(
                    crate::admin_page_path_ref::AdminPagePathRef::from(
                        table.frontend_path().as_ref(),
                    )
                ),
                Some(table)
            );
            assert!(
                table
                    .permission()
                    .as_str()
                    .get()
                    .ends_with(constants_str::VALUE_AFAFDEB2)
            );
        });
    assert_eq!(
        crate::admin_data_table::AdminDataTable::from_frontend_path(
            crate::admin_page_path_ref::AdminPagePathRef::from(constants_str::VALUE_596AD841,)
        ),
        None
    );
    assert_eq!(
        crate::admin_data_table::AdminDataTable::from_frontend_path(
            crate::admin_page_path_ref::AdminPagePathRef::from(constants_str::VALUE_D23F03FD,)
        ),
        None
    );
}

#[test]
fn test_administrator_identifiers_require_positive_database_values() {
    let _user_error = crate::admin_user_id::AdminUserId::try_from(constants_i64::ZERO)
        .expect_err(constants_str::VALUE_C3B46626);
    let _role_error = crate::admin_role_id::AdminRoleId::try_from(-constants_i64::ONE)
        .expect_err(constants_str::VALUE_4D8B8679);
    let _permission_error =
        crate::admin_permission_id::AdminPermissionId::try_from(constants_i64::ZERO)
            .expect_err(constants_str::VALUE_4556AA65);
    let _audit_error = crate::admin_audit_log_id::AdminAuditLogId::try_from(-constants_i64::ONE)
        .expect_err(constants_str::VALUE_18E48FFC);
}
