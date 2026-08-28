use super::{AxumHealthCheckStatus, HealthCheckSucceeded};

#[cfg(test)]
pub(crate) fn map_health_check_status(is_ok: HealthCheckSucceeded) -> AxumHealthCheckStatus {
    if is_ok.0 {
        AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        AxumHealthCheckStatus::from(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    }
}
