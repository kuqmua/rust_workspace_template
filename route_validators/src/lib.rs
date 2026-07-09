pub mod check_body_size;
pub mod check_commit;
pub mod hdr_val;
#[cfg(test)]
pub(crate) mod test_hlp;
//todo request per second middleware
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AxumHttpStatusCode(axum::http::StatusCode);
impl AxumHttpStatusCode {
    pub const BAD_REQUEST: Self = Self(axum::http::StatusCode::BAD_REQUEST);
    pub const IM_A_TEAPOT: Self = Self(axum::http::StatusCode::IM_A_TEAPOT);
    pub const PAYLOAD_TOO_LARGE: Self = Self(axum::http::StatusCode::PAYLOAD_TOO_LARGE);
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
            super::AxumHttpStatusCode::IM_A_TEAPOT;
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
