#[must_use]
pub fn add_health_routes(
    router: crate::axum_router::AxumRouter,
    readiness: &crate::health_readiness::HealthReadiness,
) -> crate::axum_router::AxumRouter {
    let readiness_for_route = readiness.clone();
    crate::axum_router::AxumRouter::from(
        axum::Router::from(router)
            .route(
                constants_str::catalog::LIVE_PATH,
                axum::routing::get(async || {
                    axum::Json(crate::service_liveness_snapshot::ServiceLivenessSnapshot {
                        service: crate::health_component_status::HealthComponentStatus::Ok,
                    })
                }),
            )
            .route(
                constants_str::catalog::READY_PATH,
                axum::routing::get(move || {
                    let route_readiness = readiness_for_route.clone();
                    async move {
                        let snapshot = route_readiness.snapshot();
                        if snapshot.database
                            == crate::health_component_status::HealthComponentStatus::Ok
                        {
                            Ok(axum::Json(snapshot))
                        } else {
                            Err(crate::health_ready_error::HealthReadyError::Unavailable(
                                snapshot,
                            ))
                        }
                    }
                }),
            ),
    )
}
