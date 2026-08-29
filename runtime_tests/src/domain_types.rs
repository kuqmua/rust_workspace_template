pub use super::http_runtime_test_status::HttpRuntimeTestStatus;
pub(crate) use super::reqwest_runtime_test_client::ReqwestRuntimeTestClient;
pub(crate) use super::reqwest_runtime_test_response::ReqwestRuntimeTestResponse;
pub use super::runtime_test_config::RuntimeTestConfig;
pub use super::runtime_test_error::RuntimeTestError;
pub use super::runtime_test_kind::RuntimeTestKind;
pub use super::runtime_test_report::RuntimeTestReport;
pub(crate) use super::runtime_test_url::RuntimeTestUrl;
pub use super::service_base_url::ServiceBaseUrl;
pub use super::service_base_url_error::ServiceBaseUrlError;
#[cfg(test)]
mod tests {
    #[test]
    fn service_base_url_normalizes_trailing_slashes() {
        let base_url = super::ServiceBaseUrl::try_from(String::from(constants_str::VALUE_88B6A990))
            .expect("087da3f2 service_base_url_normalizes_trailing_slashes invariant must hold");
        assert_eq!(base_url.as_ref(), "http://127.0.0.1:8080");
    }

    #[test]
    fn service_base_url_rejects_non_http_urls_and_suffixes() {
        assert_eq!(
            super::ServiceBaseUrl::try_from(String::from("postgres://database/service")),
            Err(super::ServiceBaseUrlError::Scheme)
        );
        assert_eq!(
            super::ServiceBaseUrl::try_from(String::from("http://service/path?query=true")),
            Err(super::ServiceBaseUrlError::Suffix)
        );
    }
}
