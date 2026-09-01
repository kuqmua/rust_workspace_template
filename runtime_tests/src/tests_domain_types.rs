#[cfg(test)]
mod tests {
    #[test]
    fn test_service_base_url_normalizes_trailing_slashes() {
        let base_url = crate::service_base_url::ServiceBaseUrl::try_from(String::from(
            constants_str::VALUE_88B6A990,
        ))
        .expect(constants_str::DIAGNOSTIC_087DA3F2);
        assert_eq!(base_url.as_ref(), "http://127.0.0.1:8080");
    }

    #[test]
    fn test_service_base_url_rejects_non_http_urls_and_suffixes() {
        assert_eq!(
            crate::service_base_url::ServiceBaseUrl::try_from(String::from(
                "postgres://database/service"
            )),
            Err(crate::service_base_url_error::ServiceBaseUrlError::Scheme)
        );
        assert_eq!(
            crate::service_base_url::ServiceBaseUrl::try_from(String::from(
                "http://service/path?query=true"
            )),
            Err(crate::service_base_url_error::ServiceBaseUrlError::Suffix)
        );
    }
}
