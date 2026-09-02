#[cfg(test)]
pub(crate) fn map_health_check_status(
    health_check_succeeded: crate::health_check_succeeded::HealthCheckSucceeded,
) -> crate::axum_health_check_status::AxumHealthCheckStatus {
    if bool::from(health_check_succeeded) {
        crate::axum_health_check_status::AxumHealthCheckStatus::from(axum::http::StatusCode::OK)
    } else {
        crate::axum_health_check_status::AxumHealthCheckStatus::from(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
        )
    }
}
