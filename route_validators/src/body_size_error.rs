#[location::errors_with_location]
#[derive(
    Debug, thiserror::Error, location::Location, optimal_memory_layout::OptimalMemoryLayout,
)]
#[location_to_schema]
pub enum BodySizeError {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        error: super::AxumBodySizeError,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: super::BodySizeLimitBytes,
        #[eo_to_err_string]
        size_hint: super::HttpBodySizeHint,
    },
}

impl crate::AxumHttpStatusCodeProvider for BodySizeError {
    fn axum_http_status_code(&self) -> crate::AxumHttpStatusCode {
        crate::AxumHttpStatusCode::payload_too_large()
    }
}
