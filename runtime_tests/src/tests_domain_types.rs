#[cfg(test)]
mod tests {
    #[test]
    fn test_service_base_url_normalizes_trailing_slashes() {
        let base_url = crate::service_base_url::ServiceBaseUrl::try_from(String::from(
            constants_str::VALUE_88B6A990,
        ))
        .expect(constants_str::DIAGNOSTIC_087DA3F2);
        assert_eq!(base_url.as_ref(), constants_str::VALUE_D30A576C);
    }

    #[test]
    fn test_service_base_url_rejects_non_http_urls_and_suffixes() {
        assert_eq!(
            crate::service_base_url::ServiceBaseUrl::try_from(String::from(
                constants_str::VALUE_A22A210E
            )),
            Err(crate::service_base_url_error::ServiceBaseUrlError::Scheme)
        );
        assert_eq!(
            crate::service_base_url::ServiceBaseUrl::try_from(String::from(
                constants_str::VALUE_9380378A
            )),
            Err(crate::service_base_url_error::ServiceBaseUrlError::Suffix)
        );
    }
}
