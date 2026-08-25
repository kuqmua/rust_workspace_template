fn state(pool: sqlx::PgPool) -> super::NotificationState {
    super::NotificationState {
        metrics: super::MetricsExporterPrometheusHandle::from(
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .build_recorder()
                .handle(),
        ),
        pool: app_state::domain_types::SqlxPgPool::from(pool),
        project_git_info: git_info::domain_types::project_git_info(),
    }
}

#[tokio::test]
async fn boundary_adapters_preserve_status_state_and_exit_code() {
    let status_response = axum::response::IntoResponse::into_response(
        super::HttpNotificationStatusCode::from(http::StatusCode::IM_A_TEAPOT),
    );
    assert_eq!(status_response.status(), http::StatusCode::IM_A_TEAPOT);
    assert_eq!(
        std::process::Termination::report(super::NotificationExitCode::from(
            std::process::ExitCode::SUCCESS,
        )),
        std::process::ExitCode::SUCCESS
    );

    let state = state(
        sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(
                constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .expect("75b0f8e4 boundary_adapters_preserve_status_state_and_exit_code invariant must hold"),
    );
    let request = http::Request::new(());
    let (mut parts, _body) = request.into_parts();
    let extracted = <super::AxumNotificationState as axum::extract::FromRequestParts<
        super::NotificationState,
    >>::from_request_parts(&mut parts, &state)
    .await
    .expect("c12d49a7 boundary_adapters_preserve_status_state_and_exit_code invariant must hold");
    assert_eq!(
        AsRef::<str>::as_ref(extracted.get()),
        AsRef::<str>::as_ref(&state)
    );
    let _pool = app_state::domain_types::SqlxPgPoolProvider::sqlx_pg_pool(&state);
}

#[tokio::test]
#[cfg_attr(
    miri,
    ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
)]
async fn default_service_routes_return_success_statuses() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect("52a25be1 default_service_routes_return_success_statuses invariant must hold");
    let router = crate::adapters::routes::router(
        state(pool),
        super::NotificationBodyMaximumBytes::from(
            notification_service_contract::domain_types::NOTIFICATION_API_BODY_MAX_BYTES,
        ),
    )
    .into_inner();
    let liveness_response = tower::ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(
                common_routes::domain_types::CommonRoute::HealthLive
                    .path()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("ec467ec0 default_service_routes_return_success_statuses invariant must hold"),
    )
    .await
    .expect("717fb1f4 default_service_routes_return_success_statuses invariant must hold");
    assert_eq!(liveness_response.status(), http::StatusCode::OK);
    let open_api_response = tower::ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(constants_str::OPENAPI_JSON)
            .body(axum::body::Body::empty())
            .expect("789db8f3 default_service_routes_return_success_statuses invariant must hold"),
    )
    .await
    .expect("2d37fbd2 default_service_routes_return_success_statuses invariant must hold");
    assert_eq!(open_api_response.status(), http::StatusCode::OK);
    let metrics_response = tower::ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(
                frontend_contract::domain_types::HandlerContract::path(
                    notification_service_contract::domain_types::NotificationOperationalRoute::Metrics,
                )
                .get(),
            )
            .body(axum::body::Body::empty())
            .expect(
                "f9a73c10 default_service_routes_return_success_statuses invariant must hold",
            ),
    )
    .await
    .expect("81c4e6a2 default_service_routes_return_success_statuses invariant must hold");
    assert_eq!(metrics_response.status(), http::StatusCode::OK);

    let create_metadata = <notification_service_contract::domain_types::CreateNotificationRoute as frontend_contract::domain_types::TypedRoute>::metadata();
    let invalid_request = tower::ServiceExt::oneshot(
        router,
        http::Request::builder()
            .method(create_metadata.method().as_ref())
            .uri(create_metadata.path().as_ref())
            .header(http::header::CONTENT_TYPE, constants_str::APPLICATION_JSON)
            .body(axum::body::Body::from(constants_str::TEXT_ALT_13))
            .expect("4ac710e9 default_service_routes_return_success_statuses invariant must hold"),
    )
    .await
    .expect("d8326f1b default_service_routes_return_success_statuses invariant must hold");
    assert_eq!(
        invalid_request.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    assert!(
        invalid_request
            .extensions()
            .get::<server_runtime_http::domain_types::HttpErrorTelemetry>()
            .is_some()
    );
}

#[test]
fn open_api_has_no_unresolved_schema_references() {
    frontend_contract_validation::domain_types::openapi_validation::validate_openapi_schema_references(&{
        let mut document = crate::adapters::routes::open_api_document();
        document.merge(utoipa::openapi::OpenApi::from(
            common_routes::domain_types::CommonRoutesOpenApi::open_api(),
        ));
        document
    })
    .expect("3e63ebd8 open_api_has_no_unresolved_schema_references invariant must hold");
}

#[test]
fn open_api_operation_and_statuses_come_from_the_typed_route() {
    let metadata = <notification_service_contract::domain_types::CreateNotificationRoute as frontend_contract::domain_types::TypedRoute>::metadata();
    let document = serde_json::to_value(crate::adapters::routes::open_api_document()).expect(
        "3d8a056d open_api_operation_and_statuses_come_from_the_typed_route invariant must hold",
    );
    let operation = document
        .get(constants_str::PATHS)
        .and_then(|paths| paths.get(metadata.path().as_ref()))
        .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
        .expect("fb8bb06a open_api_operation_and_statuses_come_from_the_typed_route invariant must hold");
    assert_eq!(
        operation
            .get(constants_str::OPERATION_ID_JSON)
            .and_then(serde_json::Value::as_str),
        Some(metadata.openapi_operation_id().as_ref()),
    );
    let observed_statuses = operation
        .get(constants_str::RESPONSES)
        .and_then(serde_json::Value::as_object)
        .expect("251c95e8 open_api_operation_and_statuses_come_from_the_typed_route invariant must hold")
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
fn api_problem_preserves_server_diagnostic_but_keeps_validation_expected() {
    let server_response =
        axum::response::IntoResponse::into_response(super::CreateNotificationError::Persistence(
            server_runtime_http::domain_types::ObservedError::capture(
                super::SqlxNotificationDatabaseError::from(sqlx::Error::RowNotFound),
                server_runtime_http::domain_types::ObservedErrorCode::from(
                    super::NotificationErrorCode::Persistence.get(),
                ),
            ),
        ));
    assert_eq!(
        server_response.status(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert!(
        server_response
            .extensions()
            .get::<server_runtime_http::domain_types::HttpErrorDiagnostic>()
            .is_some()
    );
    let validation_response =
        axum::response::IntoResponse::into_response(super::CreateNotificationError::Validation);
    assert_eq!(
        validation_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    assert!(
        validation_response
            .extensions()
            .get::<server_runtime_http::domain_types::HttpErrorDiagnostic>()
            .is_none()
    );
    assert!(
        validation_response
            .extensions()
            .get::<server_runtime_http::domain_types::HttpErrorTelemetry>()
            .is_some()
    );

    let metrics_response = axum::response::IntoResponse::into_response(
        super::MetricsError::Render(server_runtime_http::domain_types::ObservedError::capture(
            server_runtime_http::domain_types::MetricsResponseBodyError,
            server_runtime_http::domain_types::ObservedErrorCode::from(
                super::NotificationErrorCode::MetricsRender.get(),
            ),
        )),
    );
    assert_eq!(
        metrics_response.status(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert!(
        metrics_response
            .extensions()
            .get::<server_runtime_http::domain_types::HttpErrorDiagnostic>()
            .is_some()
    );
}

#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn create_notification_persists_through_http_route() {
    let database_url = config_lib::domain_types::parse_required_env_var(
        config_lib::domain_types::EnvVarNameRef::from(constants_str::ENV_NAMES_DATABASE_URL),
        |error, name| format!("{error} {name}"),
        <config_lib::domain_types::DatabaseUrl as config_lib::domain_types::TryFromStdEnvVarOk>::try_from_std_env_var_ok,
        |error| error.to_string(),
    )
    .expect("b3aacb7e create_notification_persists_through_http_route invariant must hold");
    let exposed_database_url = secrecy::ExposeSecret::expose_secret(&database_url.0).as_str();
    let setup_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(exposed_database_url)
        .await
        .expect("ceff90ad create_notification_persists_through_http_route invariant must hold");
    let _schema_result = sqlx::query(sqlx::AssertSqlSafe(
        constants_str::NOTIFICATION_SERVICE_CREATE_TEST_SCHEMA_SQL.concat(),
    ))
    .execute(&setup_pool)
    .await
    .expect("59114ac3 create_notification_persists_through_http_route invariant must hold");
    setup_pool.close().await;
    let connect_options =
        <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(exposed_database_url)
            .expect("2145d54a create_notification_persists_through_http_route invariant must hold")
            .options([(
                constants_str::SEARCH_PATH,
                constants_str::NOTIFICATION_SERVICE_TEST_SCHEMA,
            )]);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2u32)
        .connect_with(connect_options)
        .await
        .expect("5344bc9e create_notification_persists_through_http_route invariant must hold");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("128c46f1 create_notification_persists_through_http_route invariant must hold");
    let message = notification_service_contract::domain_types::NotificationMessage::try_from(
        constants_str::INTEGRATION_NOTIFICATION_MESSAGE.to_owned(),
    )
    .expect("f9605432 create_notification_persists_through_http_route invariant must hold");
    let body = serde_json::to_vec(
        &notification_service_contract::domain_types::CreateNotificationReq::new(message),
    )
    .expect("3daa1ab0 create_notification_persists_through_http_route invariant must hold");
    let request = http::Request::builder()
        .method(http::Method::POST)
        .uri(
            frontend_contract::domain_types::typed_route_path::<
                notification_service_contract::domain_types::CreateNotificationRoute,
            >()
            .as_ref(),
        )
        .header(
            http::header::CONTENT_TYPE,
            constants_str::HTTP_APPLICATION_JSON,
        )
        .body(axum::body::Body::from(body))
        .expect("f8d2ab0b create_notification_persists_through_http_route invariant must hold");
    let response = tower::ServiceExt::oneshot(
        crate::adapters::routes::router(
            state(pool),
            super::NotificationBodyMaximumBytes::from(
                notification_service_contract::domain_types::NOTIFICATION_API_BODY_MAX_BYTES,
            ),
        )
        .into_inner(),
        request,
    )
    .await
    .expect("c46bf92a create_notification_persists_through_http_route invariant must hold");
    assert_eq!(response.status(), http::StatusCode::CREATED);
    let response_body = axum::body::to_bytes(response.into_body(), 16_384usize)
        .await
        .expect("0aace9dd create_notification_persists_through_http_route invariant must hold");
    let created: notification_service_contract::domain_types::CreateNotificationRes =
        serde_json::from_slice(response_body.as_ref())
            .expect("e5352eef create_notification_persists_through_http_route invariant must hold");
    assert_ne!(
        created.id(),
        notification_service_contract::domain_types::UuidNotificationId::from(uuid::Uuid::nil())
    );
}
