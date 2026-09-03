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

fn typed_operation(
    value: &serde_json::Value,
    route_metadata: frontend_contract::route_metadata::RouteMetadata,
) -> &serde_json::Value {
    value
        .get(constants_str::PATHS)
        .and_then(|paths| paths.get(route_metadata.path().as_ref()))
        .and_then(|path| path.get(route_metadata.method().as_ref().to_ascii_lowercase()))
        .expect(constants_str::DIAGNOSTIC_61B8F042)
}

fn parameter_names(value: &serde_json::Value, str: &str) -> Vec<String> {
    value
        .get(constants_str::VALUE_F528212A)
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter(|parameter| {
            parameter
                .get(constants_str::VALUE_58296753)
                .and_then(serde_json::Value::as_str)
                == Some(str)
        })
        .filter_map(|parameter| {
            parameter
                .get(constants_str::NAME)
                .and_then(serde_json::Value::as_str)
        })
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
                .get(constants_str::DOLLAR_REF)
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
fn test_generated_table_catalog_maps_every_supported_data_table_once() {
    let expected = [
        (
            crate::admin_generated_table::AdminGeneratedTable::Roles,
            server_admin_contract::admin_data_table::AdminDataTable::Roles,
        ),
        (
            crate::admin_generated_table::AdminGeneratedTable::RolePermissions,
            server_admin_contract::admin_data_table::AdminDataTable::RolePermissions,
        ),
        (
            crate::admin_generated_table::AdminGeneratedTable::Users,
            server_admin_contract::admin_data_table::AdminDataTable::Users,
        ),
        (
            crate::admin_generated_table::AdminGeneratedTable::Permissions,
            server_admin_contract::admin_data_table::AdminDataTable::Permissions,
        ),
        (
            crate::admin_generated_table::AdminGeneratedTable::SystemSettings,
            server_admin_contract::admin_data_table::AdminDataTable::SystemSettings,
        ),
        (
            crate::admin_generated_table::AdminGeneratedTable::UserRoles,
            server_admin_contract::admin_data_table::AdminDataTable::UserRoles,
        ),
    ];
    assert_eq!(
        crate::admin_generated_table::AdminGeneratedTable::ALL.len(),
        expected.len()
    );
    expected.into_iter().for_each(|(generated, data_table)| {
        assert!(crate::admin_generated_table::AdminGeneratedTable::ALL.contains(&generated));
        assert_eq!(
            crate::admin_generated_table::AdminGeneratedTable::for_data_table(data_table),
            Some(generated)
        );
    });
    [
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
        server_admin_contract::admin_data_table::AdminDataTable::AuditLog,
        server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus,
        server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts,
        server_admin_contract::admin_data_table::AdminDataTable::RateLimits,
        server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens,
    ]
    .into_iter()
    .for_each(|data_table| {
        assert_eq!(
            crate::admin_generated_table::AdminGeneratedTable::for_data_table(data_table),
            None
        );
    });
}

#[test]
fn test_generated_admin_open_api_has_no_unresolved_local_references() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_F514A558);
    assert_local_references_resolve(&document, &document);
}

#[test]
fn test_every_typed_route_path_and_each_path_parameter_match_open_api() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_AB2E610C);
    <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::route_metadata()
            .as_ref()
            .iter()
            .copied()
            .for_each(|metadata| {
                let operation = typed_operation(&document, metadata);
                assert_eq!(
                    operation.get(constants_str::OPERATION_ID_JSON).and_then(serde_json::Value::as_str),
                    Some(metadata.openapi_operation_id().as_ref()),
                    "operation id differs for {} {}",
                    metadata.method().as_ref(),
                    metadata.path().as_ref(),
                );
                let success_status = u16::from(metadata.success_status().transport_status()).to_string();
                let success_response = operation
                    .get(constants_str::RESPONSES)
                    .and_then(|responses| responses.get(success_status.as_str()))
                    .expect(constants_str::DIAGNOSTIC_021E4AF7);
                if success_status == constants_str::VALUE_FC56DBC6 {
                    assert!(success_response.get(constants_str::OPENAPI_CONTENT).is_none());
                } else {
                    assert!(success_response.pointer(constants_str::VALUE_711260BD).is_some());
                }
                let expected = metadata
                    .path()
                    .as_ref()
                    .split('{')
                    .skip(1)
                    .filter_map(|suffix| suffix.split_once('}').map(|(name, _suffix)| name.to_owned()))
                    .collect::<Vec<_>>();
                let actual = parameter_names(operation, constants_str::PATH_ALT_5);
                assert_eq!(actual, expected, "path parameters differ for {}", metadata.path().as_ref());
                actual.iter().for_each(|name| {
                    let parameter = operation
                        .get(constants_str::VALUE_F528212A)
                        .and_then(serde_json::Value::as_array)
                        .and_then(|parameters| parameters.iter().find(|parameter| {
                            parameter.get(constants_str::NAME).and_then(serde_json::Value::as_str) == Some(name)
                                && parameter.get(constants_str::VALUE_58296753).and_then(serde_json::Value::as_str) == Some(constants_str::PATH_ALT_5)
                        }))
                        .expect(constants_str::DIAGNOSTIC_7E45CD91);
                    assert_eq!(parameter.get(constants_str::REQUIRED).and_then(serde_json::Value::as_bool), Some(true));
                    assert!(parameter.get(constants_str::JSON_SCHEMA).is_some(), "missing schema for path parameter {name}");
                });
            });
}

#[test]
fn test_every_typed_route_query_parameter_matches_open_api_individually() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_D083C1A9);
    <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::route_metadata()
            .as_ref()
            .iter()
            .copied()
            .for_each(|metadata| {
                let expected: &[&str] = match metadata.openapi_operation_id().as_ref() {
                    constants_str::AUDIT_LOG_ALT | constants_str::VALUE_6476FE13 => &[constants_str::ACTION, constants_str::CREATED_AFTER, constants_str::CREATED_BEFORE, constants_str::VALUE_6A0FB903, constants_str::VALUE_5089D2D4, constants_str::LIMIT, constants_str::OFFSET_ALT, constants_str::RESOURCE, constants_str::RESOURCE_ID, constants_str::SUCCEEDED, constants_str::USER_ID, constants_str::USER_LOGIN],
                    constants_str::VALUE_AF9B619C | constants_str::VALUE_48ED1531 | constants_str::VALUE_73CF19F8 | constants_str::SESSIONS => &[constants_str::LIMIT, constants_str::OFFSET_ALT, constants_str::SEARCH_ALT, constants_str::SORT_ALT, constants_str::DIRECTION],
                    constants_str::VALUE_21303953 => &[constants_str::VALUE_2521B522, constants_str::VALUE_67B4BFF9, constants_str::VALUE_7316023B, constants_str::VALUE_5C154525, constants_str::LIMIT, constants_str::OFFSET_ALT, constants_str::SEARCH_ALT, constants_str::SORT_ALT, constants_str::DIRECTION],
                    _ => &[],
                };
                let operation = typed_operation(&document, metadata);
                let actual = parameter_names(operation, constants_str::SHARED_VALUES_QUERY);
                assert_eq!(
                    actual.iter().map(String::as_str).collect::<std::collections::BTreeSet<_>>(),
                    expected.iter().copied().collect::<std::collections::BTreeSet<_>>(),
                    "query parameters differ for {}",
                    metadata.openapi_operation_id().as_ref()
                );
                actual.iter().for_each(|name| {
                    let parameter = operation
                        .get(constants_str::VALUE_F528212A)
                        .and_then(serde_json::Value::as_array)
                        .and_then(|parameters| parameters.iter().find(|parameter| parameter.get(constants_str::NAME).and_then(serde_json::Value::as_str) == Some(name)))
                        .expect(constants_str::DIAGNOSTIC_BA482F35);
                    assert!(parameter.get(constants_str::JSON_SCHEMA).is_some(), "missing schema for query parameter {name}");
                    let schema = parameter.get(constants_str::VALUE_DF0AD6E4).expect(constants_str::DIAGNOSTIC_CF18A7D5);
                    match name.as_str() {
                        constants_str::DIRECTION => assert_eq!(
                            schema.get(constants_str::ENUM),
                            Some(&serde_json::json!(["ascending", "descending"])),
                        ),
                        constants_str::LIMIT => {
                            assert_eq!(
                                schema.get(constants_str::VALUE_692E4E5D).and_then(serde_json::Value::as_u64),
                                Some(u64::from(server_admin_contract::admin_page_limit::AdminPageLimit::MIN))
                            );
                            assert_eq!(
                                schema.get(constants_str::VALUE_8A64FF09).and_then(serde_json::Value::as_u64),
                                Some(u64::from(server_admin_contract::admin_page_limit::AdminPageLimit::MAX))
                            );
                        }
                        constants_str::OFFSET_ALT => assert_eq!(schema.get(constants_str::VALUE_692E4E5D).and_then(serde_json::Value::as_u64), Some(0)),
                        constants_str::SEARCH_ALT => assert_eq!(schema.get(constants_str::VALUE_AC1DBF51).and_then(serde_json::Value::as_u64), Some(128)),
                        constants_str::SORT_ALT => assert_eq!(schema.get(constants_str::VALUE_AC1DBF51).and_then(serde_json::Value::as_u64), Some(32)),
                        _ => {}
                    }
                });
            });
}

#[test]
fn test_proc_macro_generated_request_contracts_match_open_api_and_each_field() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_40A639B7);
    let no_body_schema = serde_json::to_value(
        <server_admin_contract::admin_no_body::AdminNoBody as utoipa::PartialSchema>::schema(),
    )
    .expect(constants_str::DIAGNOSTIC_E185E575);
    <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::schema_contracts()
            .as_ref()
            .iter()
            .for_each(|contract| {
                let metadata = contract.metadata();
                let operation = typed_operation(&document, metadata);
                let request_body = operation.get(constants_str::VALUE_FCF523FA);
                let expected_schema = contract
                    .request_schema()
                    .cloned()
                    .map(|schema| {
                        let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                        serde_json::to_value(openapi_schema)
                    })
                    .transpose()
                    .expect(constants_str::DIAGNOSTIC_506E754A)
                    .expect(constants_str::DIAGNOSTIC_EB67C5A0);
                if expected_schema == no_body_schema {
                    assert!(request_body.is_none(), "unexpected request body for {}", metadata.openapi_operation_id().as_ref());
                    return;
                }
                let reference = request_body
                    .and_then(|body| body.pointer(constants_str::VALUE_A2D81D06))
                    .and_then(serde_json::Value::as_str)
                    .expect(constants_str::DIAGNOSTIC_26D0F83B);
                let actual_schema = document.pointer(reference.trim_start_matches('#')).expect(constants_str::DIAGNOSTIC_3754BCA2);
                assert_eq!(actual_schema, &expected_schema, "request schema differs for {}", metadata.openapi_operation_id().as_ref());
                expected_schema
                    .get(constants_str::PROPERTIES)
                    .and_then(serde_json::Value::as_object)
                    .into_iter()
                    .flatten()
                    .for_each(|(property, expected)| {
                        assert_eq!(actual_schema.get(constants_str::PROPERTIES).and_then(|properties| properties.get(property)), Some(expected), "request field differs for {}.{property}", metadata.openapi_operation_id().as_ref());
                    });
            });
}

#[test]
fn test_proc_macro_generated_response_contracts_match_open_api() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_C4DDF19E);
    <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::schema_contracts()
            .as_ref()
            .iter()
            .for_each(|contract| {
                let metadata = contract.metadata();
                let status = u16::from(metadata.success_status().transport_status()).to_string();
                let actual_schema = typed_operation(&document, metadata)
                    .pointer(format!("/responses/{status}/content/application~1json/schema").as_str());
                if metadata.success_status() == frontend_contract::success_status::SuccessStatus::Code204 {
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
                    .expect(constants_str::DIAGNOSTIC_2EDB7155)
                    .expect(constants_str::DIAGNOSTIC_54D97B5D);
                assert_eq!(actual_schema, Some(&expected_schema), "response schema differs for {}", metadata.openapi_operation_id().as_ref());
            });
}

#[test]
fn test_generated_admin_open_api_combines_enabled_routes_only() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_87B2E8FB);
    let paths = document
        .get(constants_str::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(constants_str::DIAGNOSTIC_274479A7);
    assert_eq!(paths.len(), 34usize);
    assert!(paths.contains_key(constants_str::VALUE_C764A505));
    assert!(!paths.contains_key(constants_str::VALUE_F772F137));
    assert!(paths.contains_key(constants_str::VALUE_356A53CE));
    assert!(paths.contains_key(constants_str::VALUE_2A3105E4));
    assert!(paths.contains_key(constants_str::ADMIN_USERS_RM));
    assert!(paths.contains_key(constants_str::VALUE_1FB526B2));
    assert!(!paths.contains_key(constants_str::VALUE_0878EE4E));
    assert!(paths.contains_key(constants_str::ADMIN_PERMISSIONS_RM));
    assert!(paths.contains_key(constants_str::VALUE_C65AD851));
    assert!(!paths.contains_key(constants_str::VALUE_7B7625A7));
    assert!(!paths.contains_key(constants_str::VALUE_19E13078));
    assert!(paths.contains_key(constants_str::ADMIN_SYSTEM_SETTINGS_RM));
    assert!(!paths.contains_key(constants_str::VALUE_C988BF92));
    assert!(paths.contains_key(constants_str::VALUE_E40BCD1D));
    assert!(!paths.contains_key(constants_str::VALUE_6CC4B99E));
    assert!(!paths.contains_key(constants_str::VALUE_3768D146));
}
#[test]
#[allow(
    clippy::needless_for_each,
    reason = "lint suppression is required here"
)]
fn test_generated_payload_example_routes_have_contracts_and_named_clients() {
    [
        (
            crate::admin_users::AdminUsers::rm_route(),
            crate::admin_users::AdminUsers::rm_payload_example_route(),
        ),
        (
            crate::admin_users::AdminUsers::ro_route(),
            crate::admin_users::AdminUsers::ro_payload_example_route(),
        ),
        (
            crate::admin_user_roles::AdminUserRoles::rm_route(),
            crate::admin_user_roles::AdminUserRoles::rm_payload_example_route(),
        ),
        (
            crate::admin_user_roles::AdminUserRoles::ro_route(),
            crate::admin_user_roles::AdminUserRoles::ro_payload_example_route(),
        ),
        (
            crate::admin_role_permissions::AdminRolePermissions::rm_route(),
            crate::admin_role_permissions::AdminRolePermissions::rm_payload_example_route(),
        ),
        (
            crate::admin_role_permissions::AdminRolePermissions::ro_route(),
            crate::admin_role_permissions::AdminRolePermissions::ro_payload_example_route(),
        ),
        (
            crate::admin_roles::AdminRoles::rm_route(),
            crate::admin_roles::AdminRoles::rm_payload_example_route(),
        ),
        (
            crate::admin_roles::AdminRoles::ro_route(),
            crate::admin_roles::AdminRoles::ro_payload_example_route(),
        ),
        (
            crate::admin_permissions::AdminPermissions::rm_route(),
            crate::admin_permissions::AdminPermissions::rm_payload_example_route(),
        ),
        (
            crate::admin_permissions::AdminPermissions::ro_route(),
            crate::admin_permissions::AdminPermissions::ro_payload_example_route(),
        ),
        (
            crate::admin_system_settings::AdminSystemSettings::rm_route(),
            crate::admin_system_settings::AdminSystemSettings::rm_payload_example_route(),
        ),
        (
            crate::admin_system_settings::AdminSystemSettings::ro_route(),
            crate::admin_system_settings::AdminSystemSettings::ro_payload_example_route(),
        ),
    ]
    .into_iter()
    .for_each(|(operation, example)| {
        assert_eq!(
            example.as_ref(),
            format!("{}_payload_example", operation.as_ref())
        );
    });
    let contract = crate::admin_users::AdminUsersRouteContract::for_path(
        crate::admin_users::AdminUsers::rm_payload_example_route().as_ref(),
    )
    .expect(constants_str::DIAGNOSTIC_8FB87492);
    assert_eq!(
        contract.frontend_contract().method(),
        frontend_contract::route_method::RouteMethod::Get
    );
    assert!(!contract.mutates());
    [
        size_of_val(&crate::admin_users::AdminUsersFrontendApiClient::<ClientTransport>::rm_payload_example),
        size_of_val(&crate::admin_users::AdminUsersFrontendApiClient::<ClientTransport>::ro_payload_example),
        size_of_val(&crate::admin_user_roles::AdminUserRolesFrontendApiClient::<ClientTransport>::rm_payload_example),
        size_of_val(&crate::admin_user_roles::AdminUserRolesFrontendApiClient::<ClientTransport>::ro_payload_example),
        size_of_val(
            &crate::admin_role_permissions::AdminRolePermissionsFrontendApiClient::<ClientTransport>::rm_payload_example,
        ),
        size_of_val(
            &crate::admin_role_permissions::AdminRolePermissionsFrontendApiClient::<ClientTransport>::ro_payload_example,
        ),
        size_of_val(&crate::admin_roles::AdminRolesFrontendApiClient::<ClientTransport>::rm_payload_example),
        size_of_val(&crate::admin_roles::AdminRolesFrontendApiClient::<ClientTransport>::ro_payload_example),
        size_of_val(
            &crate::admin_permissions::AdminPermissionsFrontendApiClient::<ClientTransport>::rm_payload_example,
        ),
        size_of_val(
            &crate::admin_permissions::AdminPermissionsFrontendApiClient::<ClientTransport>::ro_payload_example,
        ),
        size_of_val(
            &crate::admin_system_settings::AdminSystemSettingsFrontendApiClient::<ClientTransport>::rm_payload_example,
        ),
        size_of_val(
            &crate::admin_system_settings::AdminSystemSettingsFrontendApiClient::<ClientTransport>::ro_payload_example,
        ),
    ]
    .into_iter()
    .for_each(|size| assert_eq!(size, constants_usize::ZERO));
}
#[test]
fn test_every_admin_open_api_operation_has_a_unique_identifier() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_C731D604);
    let operation_ids = document
        .get(constants_str::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(constants_str::DIAGNOSTIC_F9B402AC)
        .values()
        .filter_map(serde_json::Value::as_object)
        .flat_map(|operations| operations.values())
        .map(|operation| {
            operation
                .get(constants_str::VALUE_3EFA7ACE)
                .and_then(serde_json::Value::as_str)
                .expect(constants_str::DIAGNOSTIC_18F4AE63)
        })
        .collect::<Vec<_>>();
    let unique = operation_ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique.len(), operation_ids.len());
}
#[test]
fn test_generated_read_routes_expose_filter_sort_and_pagination_contract() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        crate::generated_open_api::generated_open_api(),
    ))
    .expect(constants_str::DIAGNOSTIC_8457A8CA);
    let paths = document
        .get(constants_str::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(constants_str::DIAGNOSTIC_44D17AB0);
    [
        constants_str::ADMIN_USERS_RM,
        constants_str::ADMIN_ROLES_RM,
        constants_str::ADMIN_PERMISSIONS_RM,
        constants_str::ADMIN_ROLE_PERMISSIONS_RM,
        constants_str::ADMIN_USER_ROLES_RM,
        constants_str::ADMIN_SYSTEM_SETTINGS_RM,
    ]
    .into_iter()
    .for_each(|path| {
        assert!(
            paths
                .get(path)
                .and_then(|item| item.get(constants_str::POST_ALT))
                .and_then(|operation| operation.get(constants_str::VALUE_FCF523FA))
                .is_some(),
            "generated read route must accept a typed query body: {path}"
        );
    });
    let schemas = document
        .pointer(constants_str::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .expect(constants_str::DIAGNOSTIC_8DCF412E);
    [
        constants_str::ADMINUSERSRMPAYLOAD,
        constants_str::ADMINROLESRMPAYLOAD,
        constants_str::ADMINPERMISSIONSRMPAYLOAD,
        constants_str::ADMINROLEPERMISSIONSRMPAYLOAD,
        constants_str::ADMINUSERROLESRMPAYLOAD,
        constants_str::ADMINSYSTEMSETTINGSRMPAYLOAD,
    ]
    .into_iter()
    .for_each(|schema_name| {
        let properties = schemas
            .get(schema_name)
            .and_then(|schema| schema.get(constants_str::PROPERTIES))
            .and_then(serde_json::Value::as_object)
            .expect(constants_str::DIAGNOSTIC_5B8BBDD1);
        [
            constants_str::WHERE_MANY,
            constants_str::SELECT_ALT_3,
            constants_str::ORDER_BY,
            constants_str::PAGINATION,
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
fn test_generated_frontend_filter_metadata_matches_api_filter_schema() {
    let fields = crate::admin_users::AdminUsers::frontend_fields();
    let login = fields
        .as_ref()
        .iter()
        .find(|field| field.name().as_ref() == constants_str::LOGIN)
        .expect(constants_str::DIAGNOSTIC_C2A69D51);
    assert_eq!(
        login.filters().to_vec(),
        [
            frontend_contract::filter_operation::FilterOperation::Eq,
            frontend_contract::filter_operation::FilterOperation::Regex,
        ]
    );
    let schema = <pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextWhere as utoipa::PartialSchema>::schema();
    let variants = serde_json::to_value(schema)
        .expect(constants_str::DIAGNOSTIC_84D658FC)
        .get(constants_str::VALUE_780713E0)
        .and_then(serde_json::Value::as_array)
        .map(Vec::len);
    assert_eq!(variants, Some(login.filters().len()));
}
