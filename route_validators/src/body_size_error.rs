#[location::errors_with_location]
#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
#[location_to_schema]
pub enum BodySizeError {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        error: crate::axum_body_size_error::AxumBodySizeError,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: crate::body_size_limit_bytes::BodySizeLimitBytes,
        #[eo_to_err_string]
        size_hint: crate::http_body_size_hint::HttpBodySizeHint,
    },
}

impl crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider for BodySizeError {
    fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode {
        crate::axum_http_status_code::AxumHttpStatusCode::payload_too_large()
    }
}
