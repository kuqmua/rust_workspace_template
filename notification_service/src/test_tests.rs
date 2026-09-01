fn state(pool: sqlx::PgPool) -> crate::notification_state::NotificationState {
    crate::notification_state::NotificationState::new(
        crate::notification_metrics_exporter_prometheus_renderer::NotificationMetricsExporterPrometheusRenderer::from(
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .build_recorder()
                .handle(),
        ),
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool),
        git_info::project_git_info_value::project_git_info_value(),
    )
}

#[tokio::test]
async fn test_boundary_adapters_preserve_status_state_and_exit_code() {
    let status_response = axum::response::IntoResponse::into_response(
        crate::http_notification_status_code::HttpNotificationStatusCode::from(
            http::StatusCode::IM_A_TEAPOT,
        ),
    );
    assert_eq!(status_response.status(), http::StatusCode::IM_A_TEAPOT);
    assert_eq!(
        std::process::Termination::report(
            crate::notification_exit_code::NotificationExitCode::from(
                std::process::ExitCode::SUCCESS,
            )
        ),
        std::process::ExitCode::SUCCESS
    );

    let state = state(
        sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(
                constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .expect(constants_str::DIAGNOSTIC_75B0F8E4),
    );
    let request = http::Request::new(());
    let (mut parts, _body) = request.into_parts();
    let extracted = <crate::notification_axum_state::NotificationAxumState as axum::extract::FromRequestParts<
        crate::notification_state::NotificationState,
    >>::from_request_parts(&mut parts, &state)
    .await
    .expect(constants_str::DIAGNOSTIC_C12D49A7);
    assert_eq!(
        AsRef::<str>::as_ref(extracted.get()),
        AsRef::<str>::as_ref(&state)
    );
    let _pool = app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider::sqlx_pg_pool(&state);
}

#[tokio::test]
#[cfg_attr(
    miri,
    ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
)]
async fn test_default_service_routes_return_success_statuses() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect(constants_str::DIAGNOSTIC_52A25BE1);
    let router = crate::build_notification_router::build_notification_router(
        state(pool),
        crate::notification_body_maximum_bytes::NotificationBodyMaximumBytes::from(
            notification_service_contract::notification_api_body_max_bytes::NOTIFICATION_API_BODY_MAX_BYTES,
        ),
    )
    .into_inner();
    let liveness_response = tower::ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(
                common_routes::common_route::CommonRoute::HealthLive
                    .path()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_EC467EC0),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_717FB1F4);
    assert_eq!(liveness_response.status(), http::StatusCode::OK);
    let open_api_response = tower::ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(constants_str::OPENAPI_JSON)
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_789DB8F3),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_2D37FBD2);
    assert_eq!(open_api_response.status(), http::StatusCode::OK);
    let metrics_response = tower::ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(
                frontend_contract::route_registration_contract::RouteRegistrationContract::path(
                    notification_service_contract::notification_operational_route::NotificationOperationalRoute::Metrics,
                )
                .get(),
            )
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_F9A73C10),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_81C4E6A2);
    assert_eq!(metrics_response.status(), http::StatusCode::OK);

    let create_metadata = <notification_service_contract::create_notification_route::CreateNotificationRoute as frontend_contract::typed_route::TypedRoute>::metadata();
    let invalid_request = tower::ServiceExt::oneshot(
        router,
        http::Request::builder()
            .method(create_metadata.method().as_ref())
            .uri(create_metadata.path().as_ref())
            .header(http::header::CONTENT_TYPE, constants_str::APPLICATION_JSON)
            .body(axum::body::Body::from(constants_str::TEXT_ALT_13))
            .expect(constants_str::DIAGNOSTIC_4AC710E9),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_D8326F1B);
    assert_eq!(
        invalid_request.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    assert!(
        invalid_request
            .extensions()
            .get::<server_runtime_http::http_error_telemetry::HttpErrorTelemetry>()
            .is_some()
    );
}

#[test]
fn test_open_api_has_no_unresolved_schema_references() {
    frontend_contract_validation::validate_openapi_schema_references::validate_openapi_schema_references(&{
        let mut document = crate::open_api_document::open_api_document();
        document.merge(utoipa::openapi::OpenApi::from(
            common_routes::common_routes_open_api::CommonRoutesOpenApi::open_api(),
        ));
        document
    })
    .expect(constants_str::DIAGNOSTIC_3E63EBD8);
}

#[test]
fn test_open_api_operation_and_statuses_come_from_the_typed_route() {
    let metadata = <notification_service_contract::create_notification_route::CreateNotificationRoute as frontend_contract::typed_route::TypedRoute>::metadata();
    let document = serde_json::to_value(crate::open_api_document::open_api_document())
        .expect(constants_str::DIAGNOSTIC_3D8A056D);
    let operation = document
        .get(constants_str::PATHS)
        .and_then(|paths| paths.get(metadata.path().as_ref()))
        .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
        .expect(constants_str::DIAGNOSTIC_FB8BB06A);
    assert_eq!(
        operation
            .get(constants_str::OPERATION_ID_JSON)
            .and_then(serde_json::Value::as_str),
        Some(metadata.openapi_operation_id().as_ref()),
    );
    let observed_statuses = operation
        .get(constants_str::RESPONSES)
        .and_then(serde_json::Value::as_object)
        .expect(constants_str::DIAGNOSTIC_251C95E8)
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let expected_statuses =
        std::iter::once(metadata.success_status().transport_status().to_string())
            .chain(
                metadata
                    .error_statuses()
                    .iter()
                    .map(|status| status.transport_status().to_string()),
            )
            .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(observed_statuses, expected_statuses);
}

#[test]
fn test_api_problem_preserves_server_diagnostic_but_keeps_validation_expected() {
    let server_response = axum::response::IntoResponse::into_response(
        crate::create_notification_error::CreateNotificationError::Persistence(
            server_observability::observed_error::ObservedError::capture(
                crate::sqlx_notification_database_error::SqlxNotificationDatabaseError::from(
                    sqlx::Error::RowNotFound,
                ),
                server_observability::observed_error_code::ObservedErrorCode::from(
                    crate::notification_error_code::NotificationErrorCode::Persistence.get(),
                ),
            ),
        ),
    );
    assert_eq!(
        server_response.status(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert!(
        server_response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
            .is_some()
    );
    let validation_response = axum::response::IntoResponse::into_response(
        crate::create_notification_error::CreateNotificationError::Validation,
    );
    assert_eq!(
        validation_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    assert!(
        validation_response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
            .is_none()
    );
    assert!(
        validation_response
            .extensions()
            .get::<server_runtime_http::http_error_telemetry::HttpErrorTelemetry>()
            .is_some()
    );

    let metrics_response =
        axum::response::IntoResponse::into_response(crate::metrics_error::MetricsError::Render(
            server_observability::observed_error::ObservedError::capture(
                server_runtime_http::metrics_response_body_error::MetricsResponseBodyError::TooLarge,
                server_observability::observed_error_code::ObservedErrorCode::from(
                    crate::notification_error_code::NotificationErrorCode::MetricsRender.get(),
                ),
            ),
        ));
    assert_eq!(
        metrics_response.status(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert!(
        metrics_response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
            .is_some()
    );
}

#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn test_create_notification_persists_through_http_route() {
    let database_url = config_lib::parse_required_env_var::parse_required_env_var(
        config_lib::env_var_name_ref::EnvVarNameRef::from(constants_str::ENV_NAMES_DATABASE_URL),
        |error, name| format!("{error} {name}"),
        <config_lib::domain_types::DatabaseUrl as config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok,
        |error| error.to_string(),
    )
    .expect(constants_str::DIAGNOSTIC_B3AACB7E);
    let exposed_database_url =
        secrecy::ExposeSecret::expose_secret(database_url.get_inner()).as_str();
    let setup_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(exposed_database_url)
        .await
        .expect(constants_str::DIAGNOSTIC_CEFF90AD);
    let _schema_result = sqlx::query(sqlx::AssertSqlSafe(
        constants_str::NOTIFICATION_SERVICE_CREATE_TEST_SCHEMA_SQL.concat(),
    ))
    .execute(&setup_pool)
    .await
    .expect(constants_str::DIAGNOSTIC_59114AC3);
    setup_pool.close().await;
    let connect_options =
        <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(exposed_database_url)
            .expect(constants_str::DIAGNOSTIC_2145D54A)
            .options([(
                constants_str::SEARCH_PATH,
                constants_str::NOTIFICATION_SERVICE_TEST_SCHEMA,
            )]);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2u32)
        .connect_with(connect_options)
        .await
        .expect(constants_str::DIAGNOSTIC_5344BC9E);
    sqlx::migrate!("../notification_service_migrations")
        .run(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_128C46F1);
    let message =
        notification_service_contract::notification_message::NotificationMessage::try_from(
            constants_str::INTEGRATION_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_F9605432);
    let body = serde_json::to_vec(
        &notification_service_contract::create_notification_req::CreateNotificationReq::new(
            message,
        ),
    )
    .expect(constants_str::DIAGNOSTIC_3DAA1AB0);
    let request = http::Request::builder()
        .method(http::Method::POST)
        .uri(
            frontend_contract::typed_route_path::typed_route_path::<
                notification_service_contract::create_notification_route::CreateNotificationRoute,
            >()
            .as_ref(),
        )
        .header(
            http::header::CONTENT_TYPE,
            constants_str::HTTP_APPLICATION_JSON,
        )
        .body(axum::body::Body::from(body))
        .expect(constants_str::DIAGNOSTIC_F8D2AB0B);
    let response = tower::ServiceExt::oneshot(
        crate::build_notification_router::build_notification_router(
            state(pool),
            crate::notification_body_maximum_bytes::NotificationBodyMaximumBytes::from(
                notification_service_contract::notification_api_body_max_bytes::NOTIFICATION_API_BODY_MAX_BYTES,
            ),
        )
        .into_inner(),
        request,
    )
    .await
    .expect(constants_str::DIAGNOSTIC_C46BF92A);
    assert_eq!(response.status(), http::StatusCode::CREATED);
    let response_body = axum::body::to_bytes(response.into_body(), 16_384usize)
        .await
        .expect(constants_str::DIAGNOSTIC_0AACE9DD);
    let created: notification_service_contract::create_notification_res::CreateNotificationRes =
        serde_json::from_slice(response_body.as_ref()).expect(constants_str::DIAGNOSTIC_E5352EEF);
    assert_ne!(
        created.id(),
        notification_service_contract::uuid_notification_id::UuidNotificationId::from(
            uuid::Uuid::nil()
        )
    );
}
