#[test]
fn reports_distinguish_liveness_and_dependency_readiness() {
    let live = super::super::HealthReport::liveness();
    assert_eq!(live.status(), super::super::HealthStatus::Ok);
    assert_eq!(live.components.0.len(), 1usize);
    let ready =
        super::super::HealthReport::readiness(super::super::HealthDatabaseAvailable::from(true));
    assert_eq!(ready.status(), super::super::HealthStatus::Ok);
    assert_eq!(ready.components.0.len(), 2usize);
    let degraded =
        super::super::HealthReport::readiness(super::super::HealthDatabaseAvailable::from(false));
    assert_eq!(degraded.status(), super::super::HealthStatus::Degraded);
    assert_eq!(
        degraded.components.0.get(1usize).expect("16ca1c84").status,
        super::super::HealthStatus::Error
    );
}

#[test]
fn components_reject_more_than_supported() {
    let component = super::super::HealthComponent {
        kind: super::super::HealthComponentKind::ServiceAvailability,
        status: super::super::HealthStatus::Ok,
    };
    assert_eq!(
        super::super::HealthComponents::try_from(vec![component, component, component]),
        Err(super::super::HealthComponentsError)
    );
}

#[test]
fn component_schema_matches_runtime_limit() {
    let schema = <super::super::HealthComponents as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema else {
        panic!("d0d44742");
    };
    assert_eq!(array.min_items, Some(0usize));
    assert_eq!(
        array.max_items,
        Some(super::super::HEALTH_COMPONENTS_MAX_LEN)
    );
}

#[test]
fn component_serde_accepts_exact_runtime_limit() {
    let first = super::super::HealthComponent {
        kind: super::super::HealthComponentKind::ServiceAvailability,
        status: super::super::HealthStatus::Ok,
    };
    let second = super::super::HealthComponent {
        kind: super::super::HealthComponentKind::DatabaseConnectivity,
        status: super::super::HealthStatus::Degraded,
    };
    let expected = super::super::HealthComponents::from([first, second]);
    let encoded = serde_json::to_value(&expected).expect("60490918");
    let decoded =
        serde_json::from_value::<super::super::HealthComponents>(encoded).expect("4363452f");
    assert_eq!(decoded, expected);
}

#[test]
fn check_status_maps_success_and_failure() {
    assert_eq!(
        super::super::map_health_check_status(super::super::HealthCheckSucceeded(true)),
        super::super::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    );
    assert_eq!(
        super::super::map_health_check_status(super::super::HealthCheckSucceeded(false)),
        super::super::AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    );
}
