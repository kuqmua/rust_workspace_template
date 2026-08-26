#![allow(clippy::arbitrary_source_item_ordering)] // generated route registry stays adjacent to its endpoints

async fn database_is_ready(
    app_state: &crate::domain_types::ArcCommonRoutesAppState,
) -> crate::domain_types::HealthCheckSucceeded {
    let pool = app_state::domain_types::SqlxPgPoolProvider::sqlx_pg_pool(app_state.get());
    let probe = async {
        sqlx::query(constants_str::COMMON_ROUTES_HEALTH_CHECK_SQL)
            .execute(pool.as_ref())
            .await
            .is_ok()
    };
    crate::domain_types::HealthCheckSucceeded::from(bool::from(
        server_runtime_http::domain_types::run_health_probe(
            server_runtime_http::domain_types::HealthProbeTimeoutDuration::from(
                crate::domain_types::HEALTH_PROBE_TIMEOUT,
            ),
            probe,
        )
        .await,
    ))
}
fn health_report_response(
    report: crate::domain_types::HealthReport,
) -> Option<crate::domain_types::JsonRes<crate::domain_types::HealthReport>> {
    match report.status() {
        crate::domain_types::HealthStatus::Ok => Some(crate::domain_types::mk_json_res(report)),
        crate::domain_types::HealthStatus::Degraded | crate::domain_types::HealthStatus::Error => {
            None
        }
    }
}
async fn readiness_report(
    app_state: &crate::domain_types::ArcCommonRoutesAppState,
) -> Option<crate::domain_types::JsonRes<crate::domain_types::HealthReport>> {
    health_report_response(crate::domain_types::HealthReport::readiness(
        crate::domain_types::HealthDatabaseAvailable::from(bool::from(
            database_is_ready(app_state).await,
        )),
    ))
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::route_registry(
    state = crate::domain_types::ArcCommonRoutesAppState,
    family = crate::domain_types::CommonRouteFamily;
    ("", "");
    schemas(
        crate::domain_types::HealthComponent,
        crate::domain_types::HealthComponentKind,
        crate::domain_types::HealthComponents,
        crate::domain_types::HealthStatus
    );
    (crate::domain_types::GitInfoRoute, git_info),
    (crate::domain_types::HealthRoute, health),
    (crate::domain_types::HealthCheckRoute, health_check),
    (crate::domain_types::HealthLiveRoute, health_live),
    (crate::domain_types::HealthReadyRoute, health_ready),
)]
#[openapi(tags((name = "service", description = "Service operational routes")))]
struct CommonRouteRegistry;

#[allow(clippy::single_call_fn)] // domain-facing OpenAPI wrapper delegates to the adapter-owned registry
pub(crate) fn open_api() -> crate::domain_types::UtoipaCommonRoutesOpenApiDocument {
    crate::domain_types::UtoipaCommonRoutesOpenApiDocument::from(CommonRouteRegistry::open_api())
}

#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete endpoint is intentionally shared by Axum and OpenAPI metadata"
)]
async fn health_live() -> Result<
    crate::domain_types::JsonRes<crate::domain_types::HealthReport>,
    crate::domain_types::HealthLiveError,
> {
    health_report_response(crate::domain_types::HealthReport::liveness())
        .ok_or(crate::domain_types::HealthLiveError::Unavailable)
}
#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete endpoint is intentionally owned by the generated route registry"
)]
async fn health_ready(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> Result<
    crate::domain_types::JsonRes<crate::domain_types::HealthReport>,
    crate::domain_types::HealthReadyError,
> {
    readiness_report(&app_state)
        .await
        .ok_or(crate::domain_types::HealthReadyError::Unavailable)
}
#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete endpoint is intentionally owned by the generated route registry"
)]
async fn health(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> Result<
    crate::domain_types::JsonRes<crate::domain_types::HealthReport>,
    crate::domain_types::HealthError,
> {
    readiness_report(&app_state)
        .await
        .ok_or(crate::domain_types::HealthError::Unavailable)
}
#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete endpoint is intentionally owned by the generated route registry"
)]
async fn health_check(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> Result<crate::domain_types::AxumHealthCheckStatus, crate::domain_types::HealthCheckError> {
    let status = crate::domain_types::map_health_check_status(database_is_ready(&app_state).await);
    if bool::from(status.is_ok()) {
        Ok(status)
    } else {
        Err(crate::domain_types::HealthCheckError::Unavailable)
    }
}
#[frontend_contract::domain_types::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "the concrete endpoint is intentionally owned by the generated route registry"
)]
async fn git_info(
    app_state: crate::domain_types::ArcCommonRoutesAppState,
) -> crate::domain_types::JsonRes<crate::domain_types::GitInfo> {
    crate::domain_types::mk_commit_json_res(
        app_state.get(),
        crate::domain_types::mk_git_info_payload,
    )
}

#[must_use]
pub fn common_routes(
    app_state_b9fc2d94: crate::domain_types::ArcCommonRoutesAppState,
) -> crate::domain_types::AxumCommonRoutes {
    crate::domain_types::AxumCommonRoutes::from(
        CommonRouteRegistry::router()
            .fallback(async |uri, axum::extract::State(app_state_19103bd5_raw)| {
                let app_state_19103bd5: crate::domain_types::ArcCommonRoutesAppState =
                    app_state_19103bd5_raw;
                crate::domain_types::CommonNotFoundError::NotFound(
                    crate::domain_types::mk_not_found_payload(
                        crate::domain_types::AxumHttpUriRef::from(&uri),
                        git_info::domain_types::GitCommitLinkProvider::git_commit_link_cow(
                            app_state_19103bd5.get(),
                        ),
                    ),
                )
            })
            .with_state(app_state_b9fc2d94),
    )
}

#[cfg(test)]
#[path = "adapters_tests.rs"]
mod tests;
