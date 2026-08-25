pub mod check_body_size;
pub mod check_commit;
pub mod hdr_val;
#[cfg(test)]
pub(crate) mod test_hlp;
//todo request per second middleware
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
pub trait AxumHttpStatusCodeProvider {
    fn axum_http_status_code(&self) -> AxumHttpStatusCode;
}
#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct TestError;
    impl super::AxumHttpStatusCodeProvider for TestError {
        fn axum_http_status_code(&self) -> super::AxumHttpStatusCode {
            super::AxumHttpStatusCode::im_a_teapot()
        }
    }
    #[test]
    fn axum_http_status_code_default_method_returns_associated_const() {
        let error = TestError;
        assert_eq!(
            super::AxumHttpStatusCodeProvider::axum_http_status_code(&error).get(),
            axum::http::StatusCode::IM_A_TEAPOT
        );
    }
}
