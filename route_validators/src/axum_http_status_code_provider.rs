pub trait AxumHttpStatusCodeProvider {
    fn axum_http_status_code(&self) -> crate::domain_types::AxumHttpStatusCode;
}

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct TestError;

    impl super::AxumHttpStatusCodeProvider for TestError {
        fn axum_http_status_code(&self) -> crate::domain_types::AxumHttpStatusCode {
            crate::domain_types::AxumHttpStatusCode::im_a_teapot()
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
