#[derive(optml::Optml, Clone, Copy)]
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

fn typed_operation(
    document: &serde_json::Value,
    metadata: frontend_contract::RouteMetadata,
) -> &serde_json::Value {
    document
        .get(str_constants::PATHS)
        .and_then(|paths| paths.get(metadata.path().as_ref()))
        .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
        .expect("61b8f042 typed_operation invariant must hold")
}

fn parameter_names(operation: &serde_json::Value, location: &str) -> Vec<String> {
    operation
        .get("parameters")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|parameter| {
            parameter.get("in").and_then(serde_json::Value::as_str) == Some(location)
        })
        .filter_map(|parameter| parameter.get("name").and_then(serde_json::Value::as_str))
        .map(str::to_owned)
        .collect()
}

fn assert_local_references_resolve(document: &serde_json::Value, value: &serde_json::Value) {
    match value {
        serde_json::Value::Array(values) => values
            .iter()
            .for_each(|child| assert_local_references_resolve(document, child)),
        serde_json::Value::Object(values) => {
            if let Some(reference) = values
                .get(str_constants::DOLLAR_REF)
                .and_then(serde_json::Value::as_str)
                .and_then(|reference| reference.strip_prefix('#'))
            {
                assert!(
                    document.pointer(reference).is_some(),
                    "unresolved local OpenAPI reference: {reference}"
                );
            }
            values
                .values()
                .for_each(|child| assert_local_references_resolve(document, child));
        }
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {}
    }
}

#[test]
fn generated_table_catalog_maps_every_supported_data_table_once() {
    let expected = [
        (
            super::AdminGeneratedTable::Roles,
            server_admin_contract::AdminDataTable::Roles,
        ),
        (
            super::AdminGeneratedTable::RolePermissions,
            server_admin_contract::AdminDataTable::RolePermissions,
        ),
        (
            super::AdminGeneratedTable::Users,
            server_admin_contract::AdminDataTable::Users,
        ),
        (
            super::AdminGeneratedTable::Permissions,
            server_admin_contract::AdminDataTable::Permissions,
        ),
        (
            super::AdminGeneratedTable::SystemSettings,
            server_admin_contract::AdminDataTable::SystemSettings,
        ),
        (
            super::AdminGeneratedTable::UserRoles,
            server_admin_contract::AdminDataTable::UserRoles,
        ),
    ];
    assert_eq!(super::AdminGeneratedTable::ALL.len(), expected.len());
    expected.into_iter().for_each(|(generated, data_table)| {
        assert!(super::AdminGeneratedTable::ALL.contains(&generated));
        assert_eq!(
            super::AdminGeneratedTable::for_data_table(data_table),
            Some(generated)
        );
    });
    [
        server_admin_contract::AdminDataTable::AccessSessions,
        server_admin_contract::AdminDataTable::AuditLog,
        server_admin_contract::AdminDataTable::CleanupStatus,
        server_admin_contract::AdminDataTable::LoginAttempts,
        server_admin_contract::AdminDataTable::RateLimits,
        server_admin_contract::AdminDataTable::RefreshTokens,
    ]
    .into_iter()
    .for_each(|data_table| {
        assert_eq!(super::AdminGeneratedTable::for_data_table(data_table), None);
    });
}

#[test]
fn generated_admin_open_api_has_no_unresolved_local_references() {
    let document = serde_json::to_value(
        utoipa::openapi::OpenApi::from(super::generated_open_api()),
    )
    .expect(
        "f514a558 generated_admin_open_api_has_no_unresolved_local_references invariant must hold",
    );
    assert_local_references_resolve(&document, &document);
}

#[test]
fn every_typed_route_path_and_each_path_parameter_match_open_api() {
    let document =
        serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
            .expect("ab2e610c every_typed_route_path_and_each_path_parameter_match_open_api invariant must hold");
    <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::route_metadata()
            .as_ref()
            .iter()
            .copied()
            .for_each(|metadata| {
                let operation = typed_operation(&document, metadata);
                assert_eq!(
                    operation.get("operationId").and_then(serde_json::Value::as_str),
                    Some(metadata.openapi_operation_id().as_ref()),
                    "operation id differs for {} {}",
                    metadata.method().as_ref(),
                    metadata.path().as_ref(),
                );
                let success_status = u16::from(metadata.success_status().transport_status()).to_string();
                let success_response = operation
                    .get("responses")
                    .and_then(|responses| responses.get(success_status.as_str()))
                    .expect("021e4af7 every_typed_route_path_and_each_path_parameter_match_open_api invariant must hold");
                if success_status == "204" {
                    assert!(success_response.get("content").is_none());
                } else {
                    assert!(success_response.pointer("/content/application~1json/schema").is_some());
                }
                let expected = metadata
                    .path()
                    .as_ref()
                    .split('{')
                    .skip(1)
                    .filter_map(|suffix| suffix.split_once('}').map(|(name, _suffix)| name.to_owned()))
                    .collect::<Vec<_>>();
                let actual = parameter_names(operation, "path");
                assert_eq!(actual, expected, "path parameters differ for {}", metadata.path().as_ref());
                actual.iter().for_each(|name| {
                    let parameter = operation
                        .get("parameters")
                        .and_then(serde_json::Value::as_array)
                        .and_then(|parameters| parameters.iter().find(|parameter| {
                            parameter.get("name").and_then(serde_json::Value::as_str) == Some(name)
                                && parameter.get("in").and_then(serde_json::Value::as_str) == Some("path")
                        }))
                        .expect("7e45cd91 every_typed_route_path_and_each_path_parameter_match_open_api invariant must hold");
                    assert_eq!(parameter.get("required").and_then(serde_json::Value::as_bool), Some(true));
                    assert!(parameter.get("schema").is_some(), "missing schema for path parameter {name}");
                });
            });
}

#[test]
fn every_typed_route_query_parameter_matches_open_api_individually() {
    let document =
        serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
            .expect("d083c1a9 every_typed_route_query_parameter_matches_open_api_individually invariant must hold");
    <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::route_metadata()
            .as_ref()
            .iter()
            .copied()
            .for_each(|metadata| {
                let expected: &[&str] = match metadata.openapi_operation_id().as_ref() {
                    "audit_log" | "export_audit_log" => &["action", "created_after", "created_before", "cursor_created_at", "cursor_id", "limit", "offset", "resource", "resource_id", "succeeded", "user_id", "user_login"],
                    "list_permissions" | "list_roles" | "list_users" | "sessions" => &["limit", "offset", "search", "sort", "direction"],
                    "read_data_table" => &["filter_field", "filter_operation", "filter_value", "filter_end", "limit", "offset", "search", "sort", "direction"],
                    _ => &[],
                };
                let operation = typed_operation(&document, metadata);
                let actual = parameter_names(operation, "query");
                assert_eq!(
                    actual.iter().map(String::as_str).collect::<std::collections::BTreeSet<_>>(),
                    expected.iter().copied().collect::<std::collections::BTreeSet<_>>(),
                    "query parameters differ for {}",
                    metadata.openapi_operation_id().as_ref()
                );
                actual.iter().for_each(|name| {
                    let parameter = operation
                        .get("parameters")
                        .and_then(serde_json::Value::as_array)
                        .and_then(|parameters| parameters.iter().find(|parameter| parameter.get("name").and_then(serde_json::Value::as_str) == Some(name)))
                        .expect("ba482f35 every_typed_route_query_parameter_matches_open_api_individually invariant must hold");
                    assert!(parameter.get("schema").is_some(), "missing schema for query parameter {name}");
                    let schema = parameter.get("schema").expect("cf18a7d5 every_typed_route_query_parameter_matches_open_api_individually invariant must hold");
                    match name.as_str() {
                        "direction" => assert_eq!(
                            schema.get("enum"),
                            Some(&serde_json::json!(["asc", "desc"])),
                        ),
                        "limit" => {
                            assert_eq!(
                                schema.get("minimum").and_then(serde_json::Value::as_u64),
                                Some(u64::from(server_admin_contract::AdminPageLimit::MIN))
                            );
                            assert_eq!(
                                schema.get("maximum").and_then(serde_json::Value::as_u64),
                                Some(u64::from(server_admin_contract::AdminPageLimit::MAX))
                            );
                        }
                        "offset" => assert_eq!(schema.get("minimum").and_then(serde_json::Value::as_u64), Some(0)),
                        "search" => assert_eq!(schema.get("maxLength").and_then(serde_json::Value::as_u64), Some(128)),
                        "sort" => assert_eq!(schema.get("maxLength").and_then(serde_json::Value::as_u64), Some(32)),
                        _ => {}
                    }
                });
            });
}

#[test]
fn proc_macro_generated_request_contracts_match_open_api_and_each_field() {
    let document =
        serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
            .expect("40a639b7 proc_macro_generated_request_contracts_match_open_api_and_each_field invariant must hold");
    let no_body_schema = serde_json::to_value(
        <server_admin_contract::AdminNoBody as utoipa::PartialSchema>::schema(),
    )
    .expect("e185e575 proc_macro_generated_request_contracts_match_open_api_and_each_field invariant must hold");
    <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::schema_contracts()
            .as_ref()
            .iter()
            .for_each(|contract| {
                let metadata = contract.metadata();
                let operation = typed_operation(&document, metadata);
                let request_body = operation.get("requestBody");
                let expected_schema = contract
                    .request_schema()
                    .cloned()
                    .map(|schema| {
                        let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                        serde_json::to_value(openapi_schema)
                    })
                    .transpose()
                    .expect("506e754a proc_macro_generated_request_contracts_match_open_api_and_each_field invariant must hold")
                    .expect("eb67c5a0 proc_macro_generated_request_contracts_match_open_api_and_each_field invariant must hold");
                if expected_schema == no_body_schema {
                    assert!(request_body.is_none(), "unexpected request body for {}", metadata.openapi_operation_id().as_ref());
                    return;
                }
                let reference = request_body
                    .and_then(|body| body.pointer("/content/application~1json/schema/$ref"))
                    .and_then(serde_json::Value::as_str)
                    .expect("26d0f83b proc_macro_generated_request_contracts_match_open_api_and_each_field invariant must hold");
                let actual_schema = document.pointer(reference.trim_start_matches('#')).expect("3754bca2 proc_macro_generated_request_contracts_match_open_api_and_each_field invariant must hold");
                assert_eq!(actual_schema, &expected_schema, "request schema differs for {}", metadata.openapi_operation_id().as_ref());
                expected_schema
                    .get(str_constants::PROPERTIES)
                    .and_then(serde_json::Value::as_object)
                    .into_iter()
                    .flatten()
                    .for_each(|(property, expected)| {
                        assert_eq!(actual_schema.get(str_constants::PROPERTIES).and_then(|properties| properties.get(property)), Some(expected), "request field differs for {}.{property}", metadata.openapi_operation_id().as_ref());
                    });
            });
}

#[test]
fn proc_macro_generated_response_contracts_match_open_api() {
    let document = serde_json::to_value(
        utoipa::openapi::OpenApi::from(super::generated_open_api()),
    )
    .expect("c4ddf19e proc_macro_generated_response_contracts_match_open_api invariant must hold");
    <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::schema_contracts()
            .as_ref()
            .iter()
            .for_each(|contract| {
                let metadata = contract.metadata();
                let status = u16::from(metadata.success_status().transport_status()).to_string();
                let actual_schema = typed_operation(&document, metadata)
                    .pointer(format!("/responses/{status}/content/application~1json/schema").as_str());
                if metadata.success_status() == frontend_contract::SuccessStatus::Code204 {
                    assert!(actual_schema.is_none(), "unexpected response body for {}", metadata.openapi_operation_id().as_ref());
                    return;
                }
                let expected_schema = contract
                    .response_schema()
                    .cloned()
                    .map(|schema| {
                        let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                        serde_json::to_value(openapi_schema)
                    })
                    .transpose()
                    .expect("2edb7155 proc_macro_generated_response_contracts_match_open_api invariant must hold")
                    .expect("54d97b5d proc_macro_generated_response_contracts_match_open_api invariant must hold");
                assert_eq!(actual_schema, Some(&expected_schema), "response schema differs for {}", metadata.openapi_operation_id().as_ref());
            });
}

#[test]
fn generated_admin_open_api_combines_enabled_routes_only() {
    let document = serde_json::to_value(
        utoipa::openapi::OpenApi::from(super::generated_open_api()),
    )
    .expect("87b2e8fb generated_admin_open_api_combines_enabled_routes_only invariant must hold");
    let paths = document
        .get(str_constants::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(
            "274479a7 generated_admin_open_api_combines_enabled_routes_only invariant must hold",
        );
    assert_eq!(paths.len(), 34usize);
    assert!(paths.contains_key("/auth/sign_in"));
    assert!(!paths.contains_key("/auth/mfa"));
    assert!(paths.contains_key("/auth/sessions/{session_id}"));
    assert!(paths.contains_key("/users/{user_id}/password"));
    assert!(paths.contains_key("/admin_users/rm"));
    assert!(paths.contains_key("/admin_users/ro"));
    assert!(!paths.contains_key("/admin_users/cm"));
    assert!(paths.contains_key("/admin_permissions/rm"));
    assert!(paths.contains_key("/admin_permissions/ro"));
    assert!(!paths.contains_key("/admin_permissions/cm"));
    assert!(!paths.contains_key("/admin_permissions/dm"));
    assert!(paths.contains_key("/admin_system_settings/rm"));
    assert!(!paths.contains_key("/admin_system_settings/um"));
    assert!(paths.contains_key("/system_settings"));
    assert!(!paths.contains_key("/admin_system_settings/cm"));
    assert!(!paths.contains_key("/admin_system_settings/dm"));
}
#[test]
#[allow(clippy::needless_for_each)] // exhaustive generated-route assertions follow the workspace no-for-loop policy
fn generated_payload_example_routes_have_contracts_and_named_clients() {
    [
        (
            super::AdminUsers::rm_route(),
            super::AdminUsers::rm_payload_example_route(),
        ),
        (
            super::AdminUsers::ro_route(),
            super::AdminUsers::ro_payload_example_route(),
        ),
        (
            super::AdminUserRoles::rm_route(),
            super::AdminUserRoles::rm_payload_example_route(),
        ),
        (
            super::AdminUserRoles::ro_route(),
            super::AdminUserRoles::ro_payload_example_route(),
        ),
        (
            super::AdminRolePermissions::rm_route(),
            super::AdminRolePermissions::rm_payload_example_route(),
        ),
        (
            super::AdminRolePermissions::ro_route(),
            super::AdminRolePermissions::ro_payload_example_route(),
        ),
        (
            super::AdminRoles::rm_route(),
            super::AdminRoles::rm_payload_example_route(),
        ),
        (
            super::AdminRoles::ro_route(),
            super::AdminRoles::ro_payload_example_route(),
        ),
        (
            super::AdminPermissions::rm_route(),
            super::AdminPermissions::rm_payload_example_route(),
        ),
        (
            super::AdminPermissions::ro_route(),
            super::AdminPermissions::ro_payload_example_route(),
        ),
        (
            super::AdminSystemSettings::rm_route(),
            super::AdminSystemSettings::rm_payload_example_route(),
        ),
        (
            super::AdminSystemSettings::ro_route(),
            super::AdminSystemSettings::ro_payload_example_route(),
        ),
    ]
    .into_iter()
    .for_each(|(operation, example)| {
        assert_eq!(
            example.as_ref(),
            format!("{}_payload_example", operation.as_ref())
        );
    });
    let contract = super::AdminUsersRouteContract::for_path(
        super::AdminUsers::rm_payload_example_route().as_ref(),
    )
    .expect("8fb87492 generated_payload_example_routes_have_contracts_and_named_clients invariant must hold");
    assert_eq!(
        contract.frontend_contract().method(),
        frontend_contract::HttpMethod::Get
    );
    assert!(!contract.mutates());
    [
        size_of_val(&super::AdminUsersFrontendApiClient::<ClientTransport>::rm_payload_example),
        size_of_val(&super::AdminUsersFrontendApiClient::<ClientTransport>::ro_payload_example),
        size_of_val(&super::AdminUserRolesFrontendApiClient::<ClientTransport>::rm_payload_example),
        size_of_val(&super::AdminUserRolesFrontendApiClient::<ClientTransport>::ro_payload_example),
        size_of_val(
            &super::AdminRolePermissionsFrontendApiClient::<ClientTransport>::rm_payload_example,
        ),
        size_of_val(
            &super::AdminRolePermissionsFrontendApiClient::<ClientTransport>::ro_payload_example,
        ),
        size_of_val(&super::AdminRolesFrontendApiClient::<ClientTransport>::rm_payload_example),
        size_of_val(&super::AdminRolesFrontendApiClient::<ClientTransport>::ro_payload_example),
        size_of_val(
            &super::AdminPermissionsFrontendApiClient::<ClientTransport>::rm_payload_example,
        ),
        size_of_val(
            &super::AdminPermissionsFrontendApiClient::<ClientTransport>::ro_payload_example,
        ),
        size_of_val(
            &super::AdminSystemSettingsFrontendApiClient::<ClientTransport>::rm_payload_example,
        ),
        size_of_val(
            &super::AdminSystemSettingsFrontendApiClient::<ClientTransport>::ro_payload_example,
        ),
    ]
    .into_iter()
    .for_each(|size| assert_eq!(size, 0usize));
}
#[test]
fn every_admin_open_api_operation_has_a_unique_identifier() {
    let document = serde_json::to_value(
        utoipa::openapi::OpenApi::from(super::generated_open_api()),
    )
    .expect("c731d604 every_admin_open_api_operation_has_a_unique_identifier invariant must hold");
    let operation_ids = document
        .get(str_constants::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect("f9b402ac every_admin_open_api_operation_has_a_unique_identifier invariant must hold")
        .values()
        .filter_map(serde_json::Value::as_object)
        .flat_map(|operations| operations.values())
        .map(|operation| {
            operation
                .get("operationId")
                .and_then(serde_json::Value::as_str)
                .expect("18f4ae63 every_admin_open_api_operation_has_a_unique_identifier invariant must hold")
        })
        .collect::<Vec<_>>();
    let unique = operation_ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique.len(), operation_ids.len());
}
#[test]
fn generated_read_routes_expose_filter_sort_and_pagination_contract() {
    let document =
        serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
            .expect("8457a8ca generated_read_routes_expose_filter_sort_and_pagination_contract invariant must hold");
    let paths = document
        .get(str_constants::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect("44d17ab0 generated_read_routes_expose_filter_sort_and_pagination_contract invariant must hold");
    [
        str_constants::ADMIN_USERS_RM,
        str_constants::ADMIN_ROLES_RM,
        str_constants::ADMIN_PERMISSIONS_RM,
        str_constants::ADMIN_ROLE_PERMISSIONS_RM,
        str_constants::ADMIN_USER_ROLES_RM,
        str_constants::ADMIN_SYSTEM_SETTINGS_RM,
    ]
    .into_iter()
    .for_each(|path| {
        assert!(
            paths
                .get(path)
                .and_then(|item| item.get("post"))
                .and_then(|operation| operation.get("requestBody"))
                .is_some(),
            "generated read route must accept a typed query body: {path}"
        );
    });
    let schemas = document
        .pointer(str_constants::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .expect("8dcf412e generated_read_routes_expose_filter_sort_and_pagination_contract invariant must hold");
    [
        str_constants::ADMINUSERSRMPAYLOAD,
        str_constants::ADMINROLESRMPAYLOAD,
        str_constants::ADMINPERMISSIONSRMPAYLOAD,
        str_constants::ADMINROLEPERMISSIONSRMPAYLOAD,
        str_constants::ADMINUSERROLESRMPAYLOAD,
        str_constants::ADMINSYSTEMSETTINGSRMPAYLOAD,
    ]
    .into_iter()
    .for_each(|schema_name| {
        let properties = schemas
            .get(schema_name)
            .and_then(|schema| schema.get(str_constants::PROPERTIES))
            .and_then(serde_json::Value::as_object)
            .expect("5b8bbdd1 generated_read_routes_expose_filter_sort_and_pagination_contract invariant must hold");
        [
            str_constants::WHERE_MANY,
            str_constants::SELECT_ALT_3,
            str_constants::ORDER_BY,
            str_constants::PAGINATION,
        ]
        .into_iter()
        .for_each(|property| {
            assert!(
                properties.contains_key(property),
                "{schema_name} must expose {property}"
            );
        });
    });
}
#[test]
fn generated_frontend_filter_metadata_matches_api_filter_schema() {
    let fields = super::AdminUsers::frontend_fields();
    let login = fields
        .as_ref()
        .iter()
        .find(|field| field.name().as_ref() == str_constants::LOGIN)
        .expect("c2a69d51 generated_frontend_filter_metadata_matches_api_filter_schema invariant must hold");
    assert_eq!(
        login.filters().to_vec(),
        vec![
            frontend_contract::FilterOperation::Eq,
            frontend_contract::FilterOperation::Regex,
        ]
    );
    let schema = <pg_types_text_misc::StringAsNonNullTextWhere as utoipa::PartialSchema>::schema();
    let variants = serde_json::to_value(schema)
        .expect("84d658fc generated_frontend_filter_metadata_matches_api_filter_schema invariant must hold")
        .get("oneOf")
        .and_then(serde_json::Value::as_array)
        .map(Vec::len);
    assert_eq!(variants, Some(login.filters().len()));
}
