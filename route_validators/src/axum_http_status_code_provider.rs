pub trait AxumHttpStatusCodeProvider {
    fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode;
}

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct TestError;

    impl crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider for TestError {
        fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode {
            crate::axum_http_status_code::AxumHttpStatusCode::im_a_teapot()
        }
    }

    #[test]
    fn test_axum_http_status_code_default_method_returns_associated_const() {
        let error = TestError;
        assert_eq!(
            crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider::axum_http_status_code(&error).get(),
            axum::http::StatusCode::IM_A_TEAPOT
        );
    }
}
