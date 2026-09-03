#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct ServiceBaseUrl(String);

impl TryFrom<String> for ServiceBaseUrl {
    type Error = crate::service_base_url_error::ServiceBaseUrlError;

    fn try_from(mut string: String) -> Result<Self, Self::Error> {
        if string.len() > constants_usize::VALUE_8_192 {
            return Err(crate::service_base_url_error::ServiceBaseUrlError::Length);
        }
        while string.ends_with('/') {
            let _removed = string.pop();
        }
        let parsed = match reqwest::Url::parse(string.as_str()) {
            Ok(parsed) => parsed,
            Err(_error)
                if string.starts_with(constants_str::VALUE_8C8DAC95)
                    || string.starts_with(constants_str::VALUE_66DFEEED) =>
            {
                return Err(crate::service_base_url_error::ServiceBaseUrlError::Host);
            }
            Err(_error) => return Err(crate::service_base_url_error::ServiceBaseUrlError::Scheme),
        };
        if parsed.scheme() != constants_str::HTTP && parsed.scheme() != constants_str::HTTPS {
            return Err(crate::service_base_url_error::ServiceBaseUrlError::Scheme);
        }
        if parsed.host().is_none() {
            return Err(crate::service_base_url_error::ServiceBaseUrlError::Host);
        }
        if parsed.query().is_some() || parsed.fragment().is_some() {
            return Err(crate::service_base_url_error::ServiceBaseUrlError::Suffix);
        }
        Ok(Self(string))
    }
}
