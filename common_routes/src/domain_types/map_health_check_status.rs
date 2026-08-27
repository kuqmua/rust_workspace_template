use super::{AxumHealthCheckStatus, HealthCheckSucceeded};

#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
pub(crate) fn map_health_check_status(is_ok: HealthCheckSucceeded) -> AxumHealthCheckStatus {
    if is_ok.0 {
        AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    }
}
