#[test]
fn reports_distinguish_liveness_and_dependency_readiness() {
    let live = crate::domain_types::HealthReport::liveness();
    assert_eq!(live.status(), crate::domain_types::HealthStatus::Ok);
    assert_eq!(live.components.0.len(), constants_usize::ONE);
    let ready = crate::domain_types::HealthReport::readiness(
        crate::domain_types::HealthDatabaseAvailable::from(true),
    );
    assert_eq!(ready.status(), crate::domain_types::HealthStatus::Ok);
    assert_eq!(ready.components.0.len(), 2usize);
    let degraded = crate::domain_types::HealthReport::readiness(
        crate::domain_types::HealthDatabaseAvailable::from(false),
    );
    assert_eq!(
        degraded.status(),
        crate::domain_types::HealthStatus::Degraded
    );
    assert_eq!(
        degraded
            .components
            .0
            .get(constants_usize::ONE)
            .expect(
                "16ca1c84 reports_distinguish_liveness_and_dependency_readiness invariant must hold"
            )
            .status,
        crate::domain_types::HealthStatus::Error
    );
}

#[test]
fn components_reject_more_than_supported() {
    let component = crate::domain_types::HealthComponent {
        kind: crate::domain_types::HealthComponentKind::ServiceAvailability,
        status: crate::domain_types::HealthStatus::Ok,
    };
    assert_eq!(
        crate::domain_types::HealthComponents::try_from(vec![component, component, component]),
        Err(crate::domain_types::HealthComponentsError)
    );
}

#[test]
fn component_schema_matches_runtime_limit() {
    let schema = <crate::domain_types::HealthComponents as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema else {
        panic!("d0d44742");
    };
    assert_eq!(array.min_items, Some(constants_usize::ZERO));
    assert_eq!(array.max_items, Some(crate::HEALTH_COMPONENTS_MAX_LEN));
}

#[test]
fn component_serde_accepts_exact_runtime_limit() {
    let first = crate::domain_types::HealthComponent {
        kind: crate::domain_types::HealthComponentKind::ServiceAvailability,
        status: crate::domain_types::HealthStatus::Ok,
    };
    let second = crate::domain_types::HealthComponent {
        kind: crate::domain_types::HealthComponentKind::DatabaseConnectivity,
        status: crate::domain_types::HealthStatus::Degraded,
    };
    let expected = crate::domain_types::HealthComponents::from([first, second]);
    let encoded = serde_json::to_value(&expected)
        .expect("60490918 component_serde_accepts_exact_runtime_limit invariant must hold");
    let decoded = serde_json::from_value::<crate::domain_types::HealthComponents>(encoded)
        .expect("4363452f component_serde_accepts_exact_runtime_limit invariant must hold");
    assert_eq!(decoded, expected);
}

#[test]
fn check_status_maps_success_and_failure() {
    assert_eq!(
        crate::domain_types::map_health_check_status(crate::domain_types::HealthCheckSucceeded(
            true
        )),
        crate::domain_types::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    );
    assert_eq!(
        crate::domain_types::map_health_check_status(crate::domain_types::HealthCheckSucceeded(
            false
        )),
        crate::domain_types::AxumHealthCheckStatus::from(
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        )
    );
}
