pub mod check_body_size;
pub mod check_commit;
pub mod hdr_val;
#[cfg(test)]
pub(crate) mod test_hlp;
//todo request per second middleware
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct AxumHttpStatusCode(axum::http::StatusCode);
impl AxumHttpStatusCode {
    #[must_use]
    pub fn bad_request() -> Self {
        Self::from(axum::http::StatusCode::BAD_REQUEST)
    }
    #[must_use]
    pub const fn get(self) -> axum::http::StatusCode {
        self.0
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
pub trait GetAxumHttpStatusCode {
    fn get_axum_http_status_code(&self) -> AxumHttpStatusCode;
}
#[cfg(test)]
mod tests {
    struct TestError;
    impl super::GetAxumHttpStatusCode for TestError {
        fn get_axum_http_status_code(&self) -> super::AxumHttpStatusCode {
            super::AxumHttpStatusCode::im_a_teapot()
        }
    }
    #[test]
    fn get_axum_http_status_code_default_method_returns_associated_const() {
        let error = TestError;
        assert_eq!(
            super::GetAxumHttpStatusCode::get_axum_http_status_code(&error).get(),
            axum::http::StatusCode::IM_A_TEAPOT
        );
    }
}
