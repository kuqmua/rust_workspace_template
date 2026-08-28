#[test]
fn reports_distinguish_liveness_and_dependency_readiness() {
    let live = crate::HealthReport::liveness();
    assert_eq!(live.status(), crate::HealthStatus::Ok);
    assert_eq!(live.components.0.len(), constants_usize::ONE);
    let ready = crate::HealthReport::readiness(crate::HealthDatabaseAvailable::from(true));
    assert_eq!(ready.status(), crate::HealthStatus::Ok);
    assert_eq!(ready.components.0.len(), 2usize);
    let degraded = crate::HealthReport::readiness(crate::HealthDatabaseAvailable::from(false));
    assert_eq!(degraded.status(), crate::HealthStatus::Degraded);
    assert_eq!(
        degraded
            .components
            .0
            .get(constants_usize::ONE)
            .expect(
                "16ca1c84 reports_distinguish_liveness_and_dependency_readiness invariant must hold"
            )
            .status,
        crate::HealthStatus::Error
    );
}

#[test]
fn components_reject_more_than_supported() {
    let component = crate::HealthComponent {
        kind: crate::HealthComponentKind::ServiceAvailability,
        status: crate::HealthStatus::Ok,
    };
    assert_eq!(
        crate::HealthComponents::try_from(vec![component, component, component]),
        Err(crate::HealthComponentsError)
    );
}

#[test]
fn component_schema_matches_runtime_limit() {
    let schema = <crate::HealthComponents as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema else {
        panic!("d0d44742");
    };
    assert_eq!(array.min_items, Some(constants_usize::ZERO));
    assert_eq!(array.max_items, Some(crate::HEALTH_COMPONENTS_MAX_LEN));
}

#[test]
fn component_serde_accepts_exact_runtime_limit() {
    let first = crate::HealthComponent {
        kind: crate::HealthComponentKind::ServiceAvailability,
        status: crate::HealthStatus::Ok,
    };
    let second = crate::HealthComponent {
        kind: crate::HealthComponentKind::DatabaseConnectivity,
        status: crate::HealthStatus::Degraded,
    };
    let expected = crate::HealthComponents::from([first, second]);
    let encoded = serde_json::to_value(&expected)
        .expect("60490918 component_serde_accepts_exact_runtime_limit invariant must hold");
    let decoded = serde_json::from_value::<crate::HealthComponents>(encoded)
        .expect("4363452f component_serde_accepts_exact_runtime_limit invariant must hold");
    assert_eq!(decoded, expected);
}

#[test]
fn check_status_maps_success_and_failure() {
    assert_eq!(
        crate::map_health_check_status(crate::HealthCheckSucceeded(true)),
        crate::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    );
    assert_eq!(
        crate::map_health_check_status(crate::HealthCheckSucceeded(false)),
        crate::AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    );
}
