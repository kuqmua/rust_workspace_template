#[test]
fn test_reports_distinguish_liveness_and_dependency_readiness() {
    let live = crate::health_report::HealthReport::liveness();
    assert_eq!(live.status(), crate::health_status::HealthStatus::Ok);
    assert_eq!(live.components().len(), constants_usize::ONE);
    let ready = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(true),
    );
    assert_eq!(ready.status(), crate::health_status::HealthStatus::Ok);
    assert_eq!(ready.components().len(), 2usize);
    let degraded = crate::health_report::HealthReport::readiness(
        crate::health_database_available::HealthDatabaseAvailable::from(false),
    );
    assert_eq!(
        degraded.status(),
        crate::health_status::HealthStatus::Degraded
    );
    assert_eq!(
        degraded
            .components()
            .get(constants_usize::ONE)
            .expect(
                "16ca1c84 reports_distinguish_liveness_and_dependency_readiness invariant must hold"
            )
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
        panic!("d0d44742");
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
    let encoded = serde_json::to_value(&expected)
        .expect("60490918 component_serde_accepts_exact_runtime_limit invariant must hold");
    let decoded = serde_json::from_value::<crate::health_components::HealthComponents>(encoded)
        .expect("4363452f component_serde_accepts_exact_runtime_limit invariant must hold");
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
