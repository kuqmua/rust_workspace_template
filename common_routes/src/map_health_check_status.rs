#[cfg(test)]
pub(crate) fn map_health_check_status(
    is_ok: crate::health_check_succeeded::HealthCheckSucceeded,
) -> crate::axum_health_check_status::AxumHealthCheckStatus {
    if is_ok.0 {
        crate::axum_health_check_status::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        crate::axum_health_check_status::AxumHealthCheckStatus::from(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
        )
    }
}
