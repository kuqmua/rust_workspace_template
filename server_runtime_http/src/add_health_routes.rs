#[must_use]
pub fn add_health_routes(
    router: crate::domain_types::AxumRouter,
    readiness: &super::HealthReadiness,
) -> crate::domain_types::AxumRouter {
    let readiness_for_route = readiness.clone();
    crate::domain_types::AxumRouter::from(
        axum::Router::from(router)
            .route(
                constants_str::LIVE_PATH,
                axum::routing::get(async || {
                    axum::Json(super::ServiceLivenessSnapshot {
                        service: super::HealthComponentStatus::Ok,
                    })
                }),
            )
            .route(
                constants_str::READY_PATH,
                axum::routing::get(move || {
                    let route_readiness = readiness_for_route.clone();
                    async move {
                        let snapshot = route_readiness.snapshot();
                        if snapshot.database == super::HealthComponentStatus::Ok {
                            Ok(axum::Json(snapshot))
                        } else {
                            Err(super::HealthReadyError::Unavailable(snapshot))
                        }
                    }
                }),
            ),
    )
}
