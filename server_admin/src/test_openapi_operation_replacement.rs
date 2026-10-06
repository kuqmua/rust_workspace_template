#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct TestAdminJsonWithoutBodySchemaRoute;

impl frontend_contract::typed_route::TypedRoute for TestAdminJsonWithoutBodySchemaRoute {
    type Request = server_admin_contract::admin_sign_in_request::AdminSignInRequest;
    type Response = server_admin_contract::admin_sign_in_response::AdminSignInResponse;
    type Transport = frontend_contract::public_transport::PublicTransport;

    fn metadata() -> frontend_contract::route_metadata::RouteMetadata {
        <server_admin_contract::admin_sign_in_route::AdminSignInRoute as frontend_contract::typed_route::TypedRoute>::metadata()
    }

    fn request_body() -> frontend_contract::route_request_body::RouteRequestBody {
        frontend_contract::route_request_body::RouteRequestBody::Json
    }
}

#[test]
fn test_openapi_security_helpers_replace_stale_requirements_for_actual_admin_routes() {
    let stale_operation = || {
        let mut operation = utoipa::openapi::path::OperationBuilder::new().build();
        operation.security = Some(vec![utoipa::openapi::security::SecurityRequirement::new(
            constants_str::ABC_ALT_3,
            std::iter::empty::<&str>(),
        )]);
        operation
    };
    let authenticated_scheme =
        frontend_contract::open_api_security_scheme_ref::OpenApiSecuritySchemeRef::from(
            constants_str::X,
        );
    let csrf_scheme =
        frontend_contract::open_api_security_scheme_ref::OpenApiSecuritySchemeRef::from(
            constants_str::B,
        );
    let mut public = stale_operation();
    frontend_contract::apply_openapi_security_contract::apply_openapi_security_contract::<
        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
    >(&mut public, authenticated_scheme, csrf_scheme);
    assert!(public.security.is_none());
    let mut authenticated = stale_operation();
    frontend_contract::apply_openapi_security_contract::apply_openapi_security_contract::<
        server_admin_contract::admin_me_route::AdminMeRoute,
    >(&mut authenticated, authenticated_scheme, csrf_scheme);
    assert!(
        serde_json::to_value(&authenticated.security)
            .is_ok_and(|security| { security == serde_json::json!([{ (constants_str::X): [] }]) })
    );
    let mut mutating = stale_operation();
    frontend_contract::apply_openapi_security_contract::apply_openapi_security_contract::<
        server_admin_contract::admin_update_settings_route::AdminUpdateSettingsRoute,
    >(&mut mutating, authenticated_scheme, csrf_scheme);
    assert!(
        serde_json::to_value(&mutating.security).is_ok_and(|security| {
            security == serde_json::json!([{ (constants_str::X): [], (constants_str::B): [] }])
        })
    );
}

#[test]
fn test_openapi_request_helper_removes_stale_body_and_restores_required_json_schema() {
    let mut operation = utoipa::openapi::path::OperationBuilder::new().build();
    operation.request_body = Some(utoipa::openapi::request_body::RequestBodyBuilder::new().build());
    frontend_contract::apply_openapi_request_contract::apply_openapi_request_contract::<
        server_admin_contract::admin_me_route::AdminMeRoute,
    >(&mut operation);
    assert!(operation.request_body.is_none());
    frontend_contract::apply_openapi_request_contract::apply_openapi_request_contract::<
        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
    >(&mut operation);
    assert!(operation.request_body.as_ref().is_some_and(|body| {
        assert!(matches!(body.required, Some(utoipa::openapi::Required::True)));
        let optional_schema =
            <server_admin_contract::admin_sign_in_route::AdminSignInRoute as frontend_contract::typed_route::TypedRoute>::openapi_request_body_schema();
        optional_schema.is_some_and(|route_schema| {
            let reference_schema = utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from(route_schema);
            body.content.len() == 1usize
                && body.content.get(constants_str::APPLICATION_JSON).is_some_and(|content| {
                    content.schema.as_ref().is_some_and(|schema| {
                        serde_json::to_value(schema).is_ok_and(|actual_schema| {
                            serde_json::to_value(reference_schema).is_ok_and(|expected_schema| actual_schema == expected_schema)
                        })
                    })
                })
        })
    }));
    frontend_contract::apply_openapi_request_contract::apply_openapi_request_contract::<
        TestAdminJsonWithoutBodySchemaRoute,
    >(&mut operation);
    assert!(operation.request_body.is_none());
}
