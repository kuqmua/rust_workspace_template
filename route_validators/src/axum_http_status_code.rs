#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AxumHttpStatusCode(axum::http::StatusCode);

impl AxumHttpStatusCode {
    #[must_use]
    pub fn bad_request() -> Self {
        Self::from(axum::http::StatusCode::BAD_REQUEST)
    }

    #[must_use]
    pub fn im_a_teapot() -> Self {
        Self::from(axum::http::StatusCode::IM_A_TEAPOT)
    }

    #[must_use]
    pub fn payload_too_large() -> Self {
        Self::from(axum::http::StatusCode::PAYLOAD_TOO_LARGE)
    }
}
