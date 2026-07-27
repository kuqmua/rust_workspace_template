#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdHealthProbeTimeout(std::time::Duration);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct HealthProbeSucceeded(bool);

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthComponentStatus {
    Error,
    Ok,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct HealthSnapshot {
    database: HealthComponentStatus,
    service: HealthComponentStatus,
}
#[derive(Debug, thiserror::Error)]
enum HealthReadyError {
    #[error("service is unavailable")]
    Unavailable(HealthSnapshot),
}
#[derive(Debug, thiserror::Error)]
enum HealthLiveError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ServiceLivenessSnapshot {
    service: HealthComponentStatus,
}
impl HealthSnapshot {
    #[must_use]
    pub const fn database(self) -> HealthComponentStatus {
        self.database
    }
    #[must_use]
    pub const fn service(self) -> HealthComponentStatus {
        self.service
    }
}
impl axum::response::IntoResponse for HealthReadyError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Unavailable(snapshot) => axum::response::IntoResponse::into_response((
                http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(snapshot),
            )),
        }
    }
}
impl axum::response::IntoResponse for HealthLiveError {
    fn into_response(self) -> axum::response::Response {
        match self {}
    }
}

#[derive(Clone, Debug, newtype::FromInner)]
struct StdSharedHealthReadiness(std::sync::Arc<std::sync::atomic::AtomicBool>);

#[derive(Clone, Debug)]
pub struct HealthReadiness {
    shared: StdSharedHealthReadiness,
}
impl Default for HealthReadiness {
    fn default() -> Self {
        Self {
            shared: StdSharedHealthReadiness::from(std::sync::Arc::from(
                std::sync::atomic::AtomicBool::new(false),
            )),
        }
    }
}
impl HealthReadiness {
    #[must_use]
    pub fn snapshot(&self) -> HealthSnapshot {
        let database = if self.shared.0.load(std::sync::atomic::Ordering::Acquire) {
            HealthComponentStatus::Ok
        } else {
            HealthComponentStatus::Error
        };
        HealthSnapshot {
            database,
            service: HealthComponentStatus::Ok,
        }
    }
    pub fn store_database_probe(&self, value: HealthProbeSucceeded) {
        self.shared
            .0
            .store(bool::from(value), std::sync::atomic::Ordering::Release);
    }
}

#[must_use]
pub fn add_health_routes(
    router: crate::AxumRouter,
    readiness: &HealthReadiness,
) -> crate::AxumRouter {
    let readiness_for_route = readiness.clone();
    crate::AxumRouter(
        router
            .0
            .route(
                str_constants::LIVE_PATH,
                axum::routing::get(async || {
                    Result::<_, HealthLiveError>::Ok(axum::Json(ServiceLivenessSnapshot {
                        service: HealthComponentStatus::Ok,
                    }))
                }),
            )
            .route(
                str_constants::READY_PATH,
                axum::routing::get(move || {
                    let route_readiness = readiness_for_route.clone();
                    async move {
                        let snapshot = route_readiness.snapshot();
                        if snapshot.database == HealthComponentStatus::Ok {
                            Ok(axum::Json(snapshot))
                        } else {
                            Err(HealthReadyError::Unavailable(snapshot))
                        }
                    }
                }),
            ),
    )
}
pub async fn run_health_probe<Probe>(
    timeout: StdHealthProbeTimeout,
    probe: Probe,
) -> HealthProbeSucceeded
where
    Probe: Future<Output = bool>,
{
    HealthProbeSucceeded::from(matches!(
        tokio::time::timeout(timeout.0, probe).await,
        Ok(true)
    ))
}
#[cfg(test)]
mod tests {
    #[tokio::test(start_paused = true)]
    async fn probe_distinguishes_success_failure_and_timeout() {
        let timeout = super::StdHealthProbeTimeout::from(std::time::Duration::from_secs(1u64));
        assert!(bool::from(
            super::run_health_probe(timeout, async { true }).await
        ));
        assert!(!bool::from(
            super::run_health_probe(timeout, async { false }).await
        ));
        assert!(!bool::from(
            super::run_health_probe(timeout, std::future::pending::<bool>()).await
        ));
    }

    #[test]
    fn readiness_tracks_database_probe_without_affecting_liveness() {
        let readiness = super::HealthReadiness::default();
        assert_eq!(
            readiness.snapshot().database(),
            super::HealthComponentStatus::Error
        );
        assert_eq!(
            readiness.snapshot().service(),
            super::HealthComponentStatus::Ok
        );
        readiness.store_database_probe(super::HealthProbeSucceeded::from(true));
        assert_eq!(
            readiness.snapshot().database(),
            super::HealthComponentStatus::Ok
        );
    }

    #[tokio::test]
    async fn health_routes_distinguish_live_and_ready_statuses() {
        let readiness = super::HealthReadiness::default();
        let router =
            super::add_health_routes(crate::AxumRouter::from(axum::Router::new()), &readiness).0;
        let live_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::get(str_constants::LIVE_PATH)
                .body(axum::body::Body::empty())
                .expect("a943ebaa"),
        )
        .await
        .expect("8112486b");
        assert_eq!(live_response.status(), http::StatusCode::OK);
        let unavailable_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::get(str_constants::READY_PATH)
                .body(axum::body::Body::empty())
                .expect("341e303a"),
        )
        .await
        .expect("ee4cfce6");
        assert_eq!(
            unavailable_response.status(),
            http::StatusCode::SERVICE_UNAVAILABLE
        );
        readiness.store_database_probe(super::HealthProbeSucceeded::from(true));
        let ready_response = tower::ServiceExt::oneshot(
            router,
            http::Request::get(str_constants::READY_PATH)
                .body(axum::body::Body::empty())
                .expect("67247299"),
        )
        .await
        .expect("7cf14a1f");
        assert_eq!(ready_response.status(), http::StatusCode::OK);
    }
}
