pub mod check_body_size;
pub mod check_commit;
mod hdr_val;
pub use hdr_val::AxumHeadersRef;
#[cfg(test)]
pub(crate) mod test_hlp;
//todo request per second middleware
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AxumHttpStatusCode(axum::http::StatusCode);
impl AxumHttpStatusCode {
    #[must_use]
    pub const fn from_status_code(value: axum::http::StatusCode) -> Self {
        Self(value)
    }
    #[must_use]
    pub const fn get(self) -> axum::http::StatusCode {
        self.0
    }
}
impl From<axum::http::StatusCode> for AxumHttpStatusCode {
    fn from(value: axum::http::StatusCode) -> Self {
        Self(value)
    }
}
pub trait GetAxumHttpStatusCode {
    const AXUM_HTTP_STATUS_CODE: AxumHttpStatusCode;
    fn get_axum_http_status_code(&self) -> AxumHttpStatusCode {
        Self::AXUM_HTTP_STATUS_CODE
    }
}
#[cfg(test)]
mod tests {
    struct TestEr;
    impl super::GetAxumHttpStatusCode for TestEr {
        const AXUM_HTTP_STATUS_CODE: super::AxumHttpStatusCode =
            super::AxumHttpStatusCode::from_status_code(axum::http::StatusCode::IM_A_TEAPOT);
    }
    #[test]
    fn get_axum_http_status_code_default_method_returns_associated_const() {
        let er = TestEr;
        assert_eq!(
            super::GetAxumHttpStatusCode::get_axum_http_status_code(&er).get(),
            axum::http::StatusCode::IM_A_TEAPOT
        );
    }
}
