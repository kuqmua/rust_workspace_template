#[test]
fn test_health_components_wire_empty_and_overflow_preserve_runtime_bounds() {
    let empty = serde_json::from_value::<crate::health_components::HealthComponents>(
        serde_json::Value::Array(Vec::new()),
    );
    assert!(empty.is_ok_and(|components| components.as_slice().is_empty()));
    let component = crate::health_component::HealthComponent::new(
        crate::health_component_kind::HealthComponentKind::ServiceAvailability,
        crate::health_status::HealthStatus::Ok,
    );
    let encoded = serde_json::to_value(vec![component, component, component]);
    assert!(encoded.is_ok());
    let Ok(encoded_components) = encoded else {
        return;
    };
    let expected = bounded_types::bounded_value_error::BoundedValueError::AboveMax {
        actual: bounded_types::bounded_len::BoundedLen::from(3usize),
        max: bounded_types::bounded_len::BoundedLen::from(
            crate::health_components_max_len::HEALTH_COMPONENTS_MAX_LEN,
        ),
    };
    assert!(
        serde_json::from_value::<crate::health_components::HealthComponents>(encoded_components)
            .is_err_and(|error| error.to_string() == expected.to_string())
    );
}

#[test]
fn test_health_components_wire_scalar_errors_preserve_array_expectation() {
    [
        serde_json::Value::Null,
        serde_json::Value::Bool(false),
        serde_json::Value::Number(1u8.into()),
        serde_json::Value::String(constants_str::X.to_owned()),
        serde_json::Value::Object(serde_json::Map::new()),
    ]
    .into_iter()
    .fold((), |(), value| {
        let expected = serde_json::from_value::<
            bounded_types::bounded_vec::BoundedVec<
                crate::health_component::HealthComponent,
                0,
                { crate::health_components_max_len::HEALTH_COMPONENTS_MAX_LEN },
            >,
        >(value.clone());
        let actual = serde_json::from_value::<crate::health_components::HealthComponents>(value);
        assert!(expected.is_err() && actual.is_err());
        assert_eq!(
            actual.err().map(|error| error.to_string()),
            expected.err().map(|error| error.to_string()),
        );
    });
}

#[test]
fn test_reports_distinguish_liveness_and_dependency_readiness() {
    let service_component = crate::health_component::HealthComponent::new(
        crate::health_component_kind::HealthComponentKind::ServiceAvailability,
        crate::health_status::HealthStatus::Ok,
    );
    let live = crate::health_report::HealthReport::liveness();
    assert_eq!(live.status(), crate::health_status::HealthStatus::Ok);
    assert_eq!(live.components().len(), constants_usize::ONE);
    assert_eq!(live.components(), &[service_component]);
    let ready = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(true),
    );
    assert_eq!(ready.status(), crate::health_status::HealthStatus::Ok);
    assert_eq!(ready.components().len(), 2usize);
    assert_eq!(
        ready.components(),
        &[
            service_component,
            crate::health_component::HealthComponent::new(
                crate::health_component_kind::HealthComponentKind::DatabaseConnectivity,
                crate::health_status::HealthStatus::Ok,
            ),
        ],
    );
    let degraded = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(false),
    );
    assert_eq!(
        degraded.status(),
        crate::health_status::HealthStatus::Degraded
    );
    assert_eq!(
        degraded.components(),
        &[
            service_component,
            crate::health_component::HealthComponent::new(
                crate::health_component_kind::HealthComponentKind::DatabaseConnectivity,
                crate::health_status::HealthStatus::Error,
            ),
        ],
    );
    assert_eq!(
        degraded
            .components()
            .get(constants_usize::ONE)
            .expect(constants_str::DIAGNOSTIC_16CA1C84)
            .status(),
        crate::health_status::HealthStatus::Error
    );
}

#[test]
fn test_components_reject_more_than_supported() {
    let component = crate::health_component::HealthComponent::new(
        crate::health_component_kind::HealthComponentKind::ServiceAvailability,
        crate::health_status::HealthStatus::Ok,
    );
    assert_eq!(
        crate::health_components::HealthComponents::try_from(vec![component, component, component]),
        Err(crate::health_components_error::HealthComponentsError::TooMany)
    );
}

#[test]
fn test_component_schema_matches_runtime_limit() {
    let schema = <crate::health_components::HealthComponents as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema else {
        std::panic::panic_any(constants_str::PANIC_D0D44742);
    };
    assert_eq!(array.min_items, Some(constants_usize::ZERO));
    assert_eq!(
        array.max_items,
        Some(crate::health_components_max_len::HEALTH_COMPONENTS_MAX_LEN)
    );
}

#[test]
fn test_component_serde_accepts_exact_runtime_limit() {
    let first = crate::health_component::HealthComponent::new(
        crate::health_component_kind::HealthComponentKind::ServiceAvailability,
        crate::health_status::HealthStatus::Ok,
    );
    let second = crate::health_component::HealthComponent::new(
        crate::health_component_kind::HealthComponentKind::DatabaseConnectivity,
        crate::health_status::HealthStatus::Degraded,
    );
    let expected = crate::health_components::HealthComponents::from([first, second]);
    let encoded = serde_json::to_value(&expected).expect(constants_str::DIAGNOSTIC_60490918);
    let decoded = serde_json::from_value::<crate::health_components::HealthComponents>(encoded)
        .expect(constants_str::DIAGNOSTIC_4363452F);
    assert_eq!(decoded, expected);
}

#[test]
fn test_check_status_maps_success_and_failure() {
    assert_eq!(
        crate::map_health_check_status_tests::map_health_check_status(
            crate::health_check_succeeded::HealthCheckSucceeded::from(true)
        ),
        crate::axum_health_check_status::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    );
    assert_eq!(
        crate::map_health_check_status_tests::map_health_check_status(
            crate::health_check_succeeded::HealthCheckSucceeded::from(false)
        ),
        crate::axum_health_check_status::AxumHealthCheckStatus::from(
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        )
    );
}

#[test]
fn test_health_check_status_classifies_only_ok_and_preserves_response_status() {
    [
        (axum::http::StatusCode::OK, true),
        (axum::http::StatusCode::NO_CONTENT, false),
        (axum::http::StatusCode::TEMPORARY_REDIRECT, false),
        (axum::http::StatusCode::BAD_REQUEST, false),
        (axum::http::StatusCode::SERVICE_UNAVAILABLE, false),
    ]
    .into_iter()
    .fold((), |(), (status_code, succeeded)| {
        let axum_health_check_status =
            crate::axum_health_check_status::AxumHealthCheckStatus::from(status_code);
        assert_eq!(bool::from(axum_health_check_status.is_ok()), succeeded);
        let response = axum::response::IntoResponse::into_response(axum_health_check_status);
        assert_eq!(response.status(), status_code);
    });
}

#[test]
fn test_health_status_wire_names_are_exact_and_unknown_values_are_rejected() {
    [
        (
            crate::health_status::HealthStatus::Degraded,
            stringify!(degraded),
        ),
        (crate::health_status::HealthStatus::Error, stringify!(error)),
        (crate::health_status::HealthStatus::Ok, stringify!(ok)),
    ]
    .into_iter()
    .fold((), |(), (health_status, wire_name)| {
        assert!(
            serde_json::to_value(health_status)
                .is_ok_and(|value| value.as_str() == Some(wire_name))
        );
        assert_eq!(
            serde_json::from_value::<crate::health_status::HealthStatus>(
                serde_json::Value::String(wire_name.to_owned()),
            )
            .ok(),
            Some(health_status)
        );
    });
    assert!(
        serde_json::from_value::<crate::health_status::HealthStatus>(serde_json::Value::String(
            constants_str::X.to_owned()
        ),)
        .is_err_and(|error| error.is_data())
    );
}

#[test]
fn test_health_component_kind_wire_names_are_exact_and_unknown_values_are_rejected() {
    [
        (
            crate::health_component_kind::HealthComponentKind::DatabaseConnectivity,
            stringify!(database_connectivity),
        ),
        (
            crate::health_component_kind::HealthComponentKind::ServiceAvailability,
            stringify!(service_availability),
        ),
    ]
    .into_iter()
    .fold((), |(), (health_component_kind, wire_name)| {
        assert!(
            serde_json::to_value(health_component_kind)
                .is_ok_and(|value| value.as_str() == Some(wire_name))
        );
        assert_eq!(
            serde_json::from_value::<crate::health_component_kind::HealthComponentKind>(
                serde_json::Value::String(wire_name.to_owned()),
            )
            .ok(),
            Some(health_component_kind)
        );
    });
    assert!(
        serde_json::from_value::<crate::health_component_kind::HealthComponentKind>(
            serde_json::Value::String(constants_str::X.to_owned()),
        )
        .is_err_and(|error| error.is_data())
    );
}
