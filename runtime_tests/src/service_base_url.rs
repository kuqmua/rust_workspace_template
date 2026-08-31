#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct ServiceBaseUrl(String);

impl TryFrom<String> for ServiceBaseUrl {
    type Error = crate::service_base_url_error::ServiceBaseUrlError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            return Err(crate::service_base_url_error::ServiceBaseUrlError::Length);
        }
        while value.ends_with('/') {
            let _removed = value.pop();
        }
        let parsed = match reqwest::Url::parse(value.as_str()) {
            Ok(parsed) => parsed,
            Err(_error)
                if value.starts_with(constants_str::VALUE_8C8DAC95)
                    || value.starts_with(constants_str::VALUE_66DFEEED) =>
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
        Ok(Self(value))
    }
}
