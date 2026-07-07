pub mod check_body_size;
pub mod check_commit;
mod hdr_val;
pub use hdr_val::HeadersRef;
#[cfg(test)]
pub(crate) mod test_hlp;
//todo request per second middleware
pub trait GetAxumHttpStatusCode {
    const AXUM_HTTP_STATUS_CODE: axum::http::StatusCode;
    fn get_axum_http_status_code(&self) -> axum::http::StatusCode {
        Self::AXUM_HTTP_STATUS_CODE
    }
}
#[cfg(test)]
mod tests {
    struct TestEr;
    impl super::GetAxumHttpStatusCode for TestEr {
        const AXUM_HTTP_STATUS_CODE: axum::http::StatusCode = axum::http::StatusCode::IM_A_TEAPOT;
    }
    #[test]
    fn get_axum_http_status_code_default_method_returns_associated_const() {
        let er = TestEr;
        assert_eq!(
            super::GetAxumHttpStatusCode::get_axum_http_status_code(&er),
            axum::http::StatusCode::IM_A_TEAPOT
        );
    }
}
